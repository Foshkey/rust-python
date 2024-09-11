use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    // Determine the executable extension based on the OS
    let exe_extension = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    // Path to the compiled Python binary
    let output = Command::new(format!("./python_app{}", exe_extension)).output()?;

    // Check if the command was successful
    if output.status.success() {
        println!(
            "Python script output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
