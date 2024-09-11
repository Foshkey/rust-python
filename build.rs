use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let dest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/embedded_python.rs");
    let mut file = fs::File::create(&dest_path).unwrap();

    let python_files = get_python_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("python_app"));
    writeln!(
        &mut file,
        "pub const PYTHON_APP: [(&str, &str); {}] = [",
        python_files.len()
    )
    .unwrap();

    for (file_path, module_name) in python_files {
        let code = fs::read_to_string(file_path).expect("Unable to read file");
        writeln!(&mut file, "({:?}, {:?}),", module_name, code).unwrap();
    }
    writeln!(&mut file, "];").unwrap();
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
                    .replace("\\", ".")
                    .replace(".py", "");
                files.push((file_path, module_name));
            }
        }
    }

    files
}
