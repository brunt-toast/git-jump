use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn main() {
    // Locate the temp file
    let temp_file = get_temp_dir()
        .to_str()
        .expect("Could not get temp directory")
        .to_owned()
        + "/git-jump.tmp";

    // Delete any existing output
    match try_delete_file(temp_file.clone()) {
        Ok(_) => {}
        Err(e) => eprintln!("Could not delete the temp file: {}", e),
    }

    let mut file = File::create(temp_file).expect("Could not create temp file");
    match get_repos() {
        Ok(lines) => {
            file.write_all(lines.join("\n").as_bytes())
                .expect("Could not write to temp file");
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    file.flush().expect("Could not flush temp file");
}

fn get_repos() -> Result<Vec<String>, String> {
    let stdout = Command::new("locate")
        .arg("*.git")
        .output()
        .expect("Failed to capture output")
        .stdout;
    let lines: Result<Vec<String>, std::io::Error> = stdout.lines().collect();

    // Return the lines or propagate any IO errors
    lines.map_err(|e| format!("Failed to read lines: {}", e))
}

fn get_temp_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        env::var("TEMP")
            .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string())
            .into()
    } else if cfg!(target_os = "macos") {
        env::var("TMPDIR")
            .unwrap_or_else(|_| "/tmp".to_string())
            .into()
    } else {
        // For Unix-like systems like Linux
        env::var("TMPDIR")
            .unwrap_or_else(|_| "/tmp".to_string())
            .into()
    }
}

fn try_delete_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        fs::remove_file(path)?;
    }

    Ok(())
}
