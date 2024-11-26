// Initialize CodeMirror editor
const editor = CodeMirror.fromTextArea(document.getElementById('python-script'), {
    mode: 'python',
    lineNumbers: true,
    theme: 'default',
});

// Handle Run Button Click
document.getElementById('run-button').addEventListener('click', () => {
    handleRunButtonClick();
});

// Function to handle the Run button click event
async function handleRunButtonClick() {
    const fileInput = document.getElementById('csv-file');
    const outputPre = document.getElementById('output');

    if (fileInput.files.length === 0) {
        alert('Please upload a CSV file');
        return;
    }

    const file = fileInput.files[0];
    const pythonCode = editor.getValue();

    try {
        const csvData = await readFileAsync(file);
        const result = await executePythonCode(pythonCode, csvData);
        handleExecutionResult(result, outputPre);
    } catch (error) {
        outputPre.textContent = `Error: ${error.message}`;
        console.error(error);
    }
}

// Read file content as text =
function readFileAsync(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = event => resolve(event.target.result);
        reader.onerror = error => reject(error);
        reader.readAsText(file);
    });
}

// Execute Python code by sending a POST request to the backend
async function executePythonCode(code, csvData) {
    const payload = {
        code: code,
        csv: csvData,
    };

    const response = await fetch('http://localhost:3000/run', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        throw new Error(`Server responded with status ${response.status}`);
    }

    const result = await response.json();
    return result;
}

// Handle the execution result
function handleExecutionResult(result, outputPre) {
    if (result.res.status === 'Ok') {
        outputPre.textContent = result.res.output;
    } else {
        outputPre.textContent = `Error: ${result.res.output}`;
    }
}