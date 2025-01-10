use std::{fs, path::PathBuf};
use serde::Deserialize;

fn players() -> Vec<String> { vec!["spotify".to_string()] }
fn use_active_player() -> bool { false }
fn align() -> String { "center".to_string() }
fn layout() -> String { "top-to-bottom".to_string() }
fn hidden() -> Vec<String> { vec![] }
fn progress() -> char { '󰝤' }
fn empty() -> char { '󰁱' }

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "players")]
    pub players: Vec<String>,

    #[serde(default = "use_active_player")]
    pub use_active_player: bool,

    #[serde(default = "align")]
    pub align: String,

    #[serde(default = "layout")]
    pub layout: String,

    #[serde(default = "hidden")]
    pub hidden: Vec<String>,

    #[serde(default = "progress")]
    pub progress: char,

    #[serde(default = "empty")]
    pub empty: char
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            use_active_player: use_active_player(),
            align: align(),
            layout: layout(),
            hidden: hidden(),
            progress: progress(),
            empty: empty()
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
