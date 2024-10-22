use std::io::BufRead;
use std::process::Command;

mod config;
mod fzf;
mod tempfile;

fn main() {
    tempfile::delete_temp_file();

    match get_repos() {
        Ok(lines) => tempfile::write_temp_file(lines.join("\n")),
        Err(e) => eprintln!("Error: {}", e),
    }

    let fzf_output =
        fzf::fzf(&tempfile::get_temp_file_path()).expect("Could not capture output from fzf");
    println!("{}", fzf_output);

    tempfile::delete_temp_file();
}

fn get_repos() -> Result<Vec<String>, String> {
    let conf = config::Config::from_json_file().expect("Could not load configuration");

    let stdout = Command::new("locate")
        .arg("*.git")
        .output()
        .expect("Failed to capture output")
        .stdout;
    let lines: Result<Vec<String>, std::io::Error> = stdout.lines().collect();

    // Return the lines or propagate any IO errors
    let ret_unfiltered = lines
        .map_err(|e| format!("Failed to read lines: {}", e))
        .expect("Failed to read lines");

    let ret_with_whitelist = conf.clone().filter_whitelist(ret_unfiltered);
    Ok(conf.filter_blacklist(ret_with_whitelist))
}
