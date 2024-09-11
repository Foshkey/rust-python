use pyo3::prelude::*;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/embedded_python.rs"
));

fn main() -> PyResult<()> {
    // This points to app.py in the python_app directory. It's expected that this file will contain
    // a function named "run", which will be called.
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));

    let python_result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        // Dynamically load all of the python files as modules.
        for (module_name, code) in PYTHON_APP.iter() {
            PyModule::from_code_bound(py, code, module_name, module_name)?;
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
