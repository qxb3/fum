use std::{fs, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub players: Vec<String>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: vec![
                "spotify".to_string()
            ]
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        match fs::read_to_string(path) {
            Ok(config_file) => {
                let config: Config = serde_json::from_str(&config_file)
                    .map_err(|err| format!("Failed to parse config: {err}"))?;

                Ok(config)
            },
            Err(_) => Ok(Config::default())
        }
    }
}
