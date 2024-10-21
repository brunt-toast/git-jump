use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn get_temp_file_path() -> String {
    get_temp_dir()
        .to_str()
        .expect("Could not get temp directory")
        .to_owned()
        + "/git-jump.tmp"
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

pub fn write_temp_file(content: String) {
    let mut file = File::create(get_temp_file_path()).expect("Could not create temp file");
    file.write_all(content.as_bytes())
        .expect("Could not write to temp file");
    file.flush().expect("Could not flush temp file");
}

pub fn delete_temp_file() {
    match try_delete_file(get_temp_file_path()) {
        Ok(_) => {}
        Err(e) => eprintln!("Could not delete the temp file: {}", e),
    }
}
