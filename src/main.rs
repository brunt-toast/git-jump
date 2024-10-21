use std::io::BufRead;
use std::process::Command;

fn main() {
    match get_repos() {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_repos() -> Result<Vec<String>, String> {
    let stdout = Command::new("locate")
        .arg("*.git")
        .spawn()
        .expect("ls command failed to start")
        .wait_with_output()
        .expect("Could not capture output")
        .stdout;
    let lines: Result<Vec<String>, std::io::Error> = stdout.lines().collect();

    // Return the lines or propagate any IO errors
    lines.map_err(|e| format!("Failed to read lines: {}", e))
}
