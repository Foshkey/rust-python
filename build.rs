use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Get the directory where Rust is building the target binary
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Define the path to your Python script
    let python_script = "python_app/app.py"; // Adjust to your script path

    // Determine the executable extension based on the OS
    let exe_extension = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    // Run the PyInstaller command to build the Python script
    let pyinstall = Command::new("pyinstaller")
        .args([
            "--onefile",
            "--distpath",
            out_dir.to_str().unwrap(), // Output to Rust's OUT_DIR
            python_script,
        ])
        .status()
        .expect("Failed to run PyInstaller");

    if !pyinstall.success() {
        panic!("PyInstaller build failed");
    }

    // Move the Python binary to the target directory
    let python_binary = out_dir.join(format!("app{}", exe_extension)); // Adjust for other OS
    let target_python_binary = Path::new(&out_dir)
        .join("../../../")
        .join(format!("python_app{}", exe_extension));

    // Ensure the target folder exists
    fs::create_dir_all(target_python_binary.parent().unwrap()).unwrap();

    // Copy the Python binary to the target directory
    fs::copy(python_binary, target_python_binary)
        .expect("Failed to copy Python binary to target folder");
}
