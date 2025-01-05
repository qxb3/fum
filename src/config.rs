use std::{fs, path::PathBuf, u16};
use serde::Deserialize;

fn players() -> Vec<String> { vec!["spotify".to_string()] }
fn align() -> String { "center".to_string() }
fn width() -> u16 { 20 }
fn height() -> u16 { 15 }

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "players")]
    pub players: Vec<String>,

    #[serde(default = "align")]
    pub align: String,

    #[serde(default = "width")]
    pub width: u16,

    #[serde(default = "height")]
    pub height: u16
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            align: align(),
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

                if !matches!(
                    config.align.as_str(),
                    "center" | "top" | "left" |
                    "bottom" | "right" | "top-left" |
                    "top-right" | "bottom-left" | "bottom-right"
                ) {
                    return Err("Invalid value for 'align'".to_string())
                }

                Ok(config)
            },
            Err(_) => Ok(Config::default())
        }
    }
}
