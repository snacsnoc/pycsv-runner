# pycsv-runner

**pycsv-runner** is a fun proof-of-concept project that allows running Python in the browser using the [Runner](https://github.com/ErikKaum/runner) WebAssembly runtime, allowing Python scripts to interact with CSV data using a `{{csv}}` placeholder.

This allows you to filter, transform, and analyze uploaded CSV files directly within the browser using Python.

Potential use cases:
* Client-side data filtering, sorting, and transformation
* Interactive data analysis and exploration
* Data cleaning and preprocessing tasks
* Custom aggregation and calculations on datasets
* Lightweight CSV manipulation sandbox for developers
* Finally proving to your boss that CSVs aren’t a database

## Running:
```
# Build and run backend
cargo build --release --manifest-path runner/Cargo.toml
cargo run --release --manifest-path runner/Cargo.toml 

# Serve frontend using a simple Python HTTP server
cd frontend
python3 -m http.server
```

Navigate to `http://localhost:8000`

## Usage:

* Upload a CSV file via the frontend
* Write or paste your Python script in the editor, using `{{csv}}` to reference the uploaded CSV data
* Run the script and view the output directly in your browser

### Example
Python script with `{{csv}}` usage:
```python
import csv
data = """{{csv}}"""  
reader = csv.DictReader(data.splitlines())
result = [row for row in reader]

print(result)
```
See the `examples/` directory.

## Acknowledgments
This project builds upon the work of [Runner](https://github.com/ErikKaum/runner) by Erik Kaunismäki.

### License
The original `Runner` project is licensed under the [MIT License](https://opensource.org/licenses/MIT). Contributions to this project also follow the MIT License.