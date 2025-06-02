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

    if fzf_output.len() == 0 {
        println!(".")
    } else {
        println!("{}", fzf_output);
    }

    tempfile::delete_temp_file();
}

fn get_repos() -> Result<Vec<String>, String> {
    let conf = config::Config::from_json_file().expect("Could not load configuration");

    let ret_unfiltered = get_repos_of_type(".git")
        .into_iter()
        .chain(get_repos_of_type(".svn"))
        .chain(get_repos_of_type(".hg"))
        .chain(get_repos_of_type("p4root"))
        .chain(get_repos_of_type(".p4config"))
        .chain(get_repos_of_type(".bzr"))
        .chain(get_repos_of_type("CVS"))
        .chain(get_repos_of_type("CVSROOT"))
        .chain(get_repos_of_type("$tf"))
        .chain(get_repos_of_type(".tfs"))
        .collect();

    let ret_with_whitelist = conf.clone().filter_whitelist(ret_unfiltered);
    let ret_with_blacklist = conf.filter_blacklist(ret_with_whitelist);
    Ok(ret_with_blacklist)
}

fn get_repos_of_type(repo_type: &str) -> Vec<String> {
    let git = Command::new("locate")
        .arg("*".to_owned() + repo_type)
        .output()
        .expect("Failed to capture output")
        .stdout;
    let stdout: Result<Vec<String>, std::io::Error> = git.lines().collect();
    let mut lines = stdout
        .map_err(|e| format!("Failed to read lines: {}", e))
        .expect("Failed to read lines");

    for item in lines.iter_mut() {
        if let Some(s) = item.strip_suffix(repo_type) {
            *item = s.to_string();
        }
    }

    return lines;
}
