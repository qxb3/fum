use std::{fs, path::PathBuf, u16};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "players")]
    pub players: Vec<String>,
    #[serde(default = "width")]
    pub width: u16,
    #[serde(default = "height")]
    pub height: u16
}

fn players() -> Vec<String> { vec!["spotify".to_string()] }
fn width() -> u16 { 20 }
fn height() -> u16 { 15 }

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            width: width(),
            height: height()
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
