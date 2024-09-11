use std::fs;
use std::path::Path;

use pyo3::prelude::*;

fn main() -> PyResult<()> {
    // This points to app.py in the python_app directory. It's expected that this file will contain
    // a function named "run", which will be called.
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));

    // All of the other files within the python_app directory will be loaded as modules.
    let python_files = get_python_files(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python_app"
    )));

    let python_result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        // Dynamically load all of the python files as modules.
        for (file_path, module_name) in python_files {
            let code = fs::read_to_string(file_path).expect("Unable to read file");
            PyModule::from_code_bound(py, &code, &module_name, &module_name)?;
        }

        // Load the app.py file and call the run function.
        let app: Py<PyAny> = PyModule::from_code_bound(py, py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", python_result?);
    Ok(())
}

fn get_python_files(dir: &Path) -> Vec<(String, String)> {
    get_python_files_rec(dir, dir)
}

fn get_python_files_rec(dir: &Path, base: &Path) -> Vec<(String, String)> {
    let mut files = Vec::new();
    if !dir.is_dir() {
        return files;
    }

    for entry in fs::read_dir(dir).expect("read_dir call failed").flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(get_python_files_rec(&path, base));
        } else {
            let Some(ext) = path.extension() else {
                continue;
            };
            let Some(file_name) = path.file_name() else {
                continue;
            };
            if ext == "py" && file_name != "app.py" {
                let file_path = path.to_str().unwrap().to_string();
                let module_name = path
                    .strip_prefix(base)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("/", ".")
                    .replace(".py", "");
                files.push((file_path, module_name));
            }
        }
    }

    files
}
