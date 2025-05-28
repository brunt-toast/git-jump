use std::io::BufRead;
use std::process::Command;

pub fn fzf(path: &str) -> Result<String, String> {
    let stdout = Command::new("bash")
        .arg("-c")
        .arg(
            format!(
                "cat '{}' | fzf 
                --preview \"tree '{}' -L 1\" 
                --info \"hidden\" 
                --header \"git-jump\" 
                --header-first 
                --reverse 
                --keep-right",
                path, "{}"
            )
            .replace("\n", ""),
        )
        .output()
        .expect("Failed to capture output")
        .stdout;
    let lines: Result<Vec<String>, std::io::Error> = stdout.lines().collect();

    // Return the lines or propagate any IO errors
    let lines = lines
        .map_err(|e| format!("Failed to read lines: {}", e))
        .expect("Failed to read lines");

    if lines.len() == 0 {
        return Err("fzf produced no output".to_owned());
    }

    return Ok(format!("{}", lines[0]));
}
