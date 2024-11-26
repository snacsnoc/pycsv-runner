use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::info;

use anyhow::Result;
use uuid::Uuid;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

use crate::memory_pipe;

pub struct PythonRunner {
    engine: Engine,
    linker: Linker<WasiP1Ctx>,
    module: Module,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PythonStatus {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PythonResponse {
    pub status: PythonStatus,
    pub output: Option<String>,
}

impl PythonRunner {
    pub async fn new() -> Result<Self> {
        let now = Instant::now();

        let mut config = Config::new();
        config.async_support(true);
        config.max_wasm_stack(16777216);
        config.async_stack_size(16777216);
        let engine = Engine::new(&config)?;

        let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
        preview1::add_to_linker_async(&mut linker, |t| t)?;

        let module = Module::from_file(&engine, "python-wasm/python.wasm")?;

        info!(
            "loading python runtime took: {:?}",
            Duration::from(Instant::now() - now)
        );

        Ok(PythonRunner {
            engine,
            linker,
            module,
        })
    }

    pub async fn run_code(
        &self,
        code: &str,
        csv: &str,
        print_guest_output: bool,
        id: Uuid,
    ) -> Result<PythonResponse> {
        let now = Instant::now();
        let sandbox_start = Instant::now();

        // in real life, don't use unbounded
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let stdout = memory_pipe::MemoryOutputPipe::new(1024, tx, print_guest_output, id);

        let mut wasi_ctx = WasiCtxBuilder::new();

        // std out & err
        wasi_ctx.stdout(stdout.clone());
        wasi_ctx.stderr(stdout);

        // otherwise the python runtime doesn't flush after write
        wasi_ctx.env("PYTHONUNBUFFERED", "1");

        // Replace the placeholder {{csv}} with the actual CSV data
        let code_with_csv = code.replace("{{csv}}", &Self::escape_csv(csv));

        // Run code with CSV data
        wasi_ctx.args(&["python", "-c", &code_with_csv]);

        // allow only read to this since it's shared among instances
        wasi_ctx.preopened_dir("python-wasm", ".", DirPerms::READ, FilePerms::READ)?;
        let wasi = wasi_ctx.build_p1();

        let mut store = Store::new(&self.engine, wasi);

        let instance = self
            .linker
            .instantiate_async(&mut store, &self.module)
            .await?;
        let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        let startup_duration = sandbox_start.elapsed();
        info!("sandbox {} startup time: {:?}", id, startup_duration);

        let res = start.call_async(&mut store, ()).await;

        info!(
            "sandbox {} exited {} after: {:?}",
            id,
            if res.is_ok() {
                "successfully"
            } else {
                "with error"
            },
            Duration::from(Instant::now() - now)
        );

        // Collect output from the memory pipe
        let mut all_output = String::new();
        while let Ok(Some(message)) =
            tokio::time::timeout(std::time::Duration::from_millis(10), rx.recv()).await
        {
            all_output.push_str(&message);
        }

        match res {
            Ok(_) => {
                return Ok(PythonResponse {
                    status: PythonStatus::Ok,
                    output: Some(all_output),
                });
            }
            Err(_) => {
                return Ok(PythonResponse {
                    status: PythonStatus::Error,
                    output: Some(all_output),
                });
            }
        }
    }

    fn escape_csv(csv: &str) -> String {
        csv.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\'", "\\\'")
    }
}
