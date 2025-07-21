use serde::Deserialize;
use serde_json::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    blacklist: Vec<String>,
    whitelist: Vec<String>,
    pub additional_repos: Vec<String>,
}

impl Config {
    pub fn from_json_file() -> Result<Self, Error> {
        let verbose = env::args().any(|s| s == "-v" || s == "--verbose");

        let home_dir = match home::home_dir() {
            Some(d) => d,
            None => panic!("Could not find the home directory to load config"),
        };
        let path = home_dir.join(".config").join("git-jump.json");
        if verbose {
            println!(
                "Home directory: {}",
                match home_dir.to_str() {
                    Some(s) => s,
                    None => "Unknown",
                }
            );
            println!(
                "Config path: {}",
                match path.to_str() {
                    Some(s) => s,
                    None => "Unknown",
                }
            );
        }

        if !path.exists() {
            if verbose {
                println!("Config path does not exist, using default config");
            }

            return Ok(Config {
                blacklist: Vec::new(),
                whitelist: Vec::new(),
                additional_repos: Vec::new(),
            });
        }

        let file = File::open(path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let ret: Config = serde_json::from_reader(reader)?;
        Ok(ret)
    }

    pub fn filter_whitelist(self, source: Vec<String>) -> Vec<String> {
        if self.whitelist.len() == 0 {
            return source;
        }

        source
            .into_iter()
            .filter(|s| {
                self.whitelist
                    .iter()
                    .any(|prefix| s.to_lowercase().starts_with(&prefix.to_lowercase()))
            })
            .collect()
    }

    pub fn filter_blacklist(self, source: Vec<String>) -> Vec<String> {
        if self.blacklist.len() == 0 {
            return source;
        }

        source
            .into_iter()
            .filter(|s| {
                self.blacklist
                    .iter()
                    .all(|prefix| !s.to_lowercase().starts_with(&prefix.to_lowercase()))
            })
            .collect()
    }
}
