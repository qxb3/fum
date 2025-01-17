use std::{fs, path::PathBuf};
use serde::Deserialize;

use crate::widget::{ContainerFlex, Direction, FumWidget};

use super::defaults::{align, debug, direction, flex, height, layout, players, use_active_player, width};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Center,
    Top,
    Left,
    Bottom,
    Right,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
}

impl Align {
    pub fn from_str(str: &str) -> Option<Self> {
        match str {
            "center"        => Some(Self::Center),
            "top"           => Some(Self::Top),
            "left"          => Some(Self::Left),
            "bottom"        => Some(Self::Bottom),
            "top-left"      => Some(Self::TopLeft),
            "top-right"     => Some(Self::TopRight),
            "bottom-left"   => Some(Self::BottomLeft),
            "bottom-right"  => Some(Self::BottomRight),
            _               => None
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "players")]
    pub players: Vec<String>,

    #[serde(default = "use_active_player")]
    pub use_active_player: bool,

    #[serde(default = "align")]
    pub align: Align,

    #[serde(default = "direction")]
    pub direction: Direction,

    #[serde(default = "flex")]
    pub flex: ContainerFlex,

    #[serde(default = "width")]
    pub width: u16,

    #[serde(default = "height")]
    pub height: u16,

    #[serde(default = "debug")]
    pub debug: Option<bool>,

    #[serde(default = "layout")]
    pub layout: Vec<FumWidget>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            use_active_player: use_active_player(),
            align: align(),
            direction: direction(),
            flex: flex(),
            width: width(),
            height: height(),
            debug: debug(),
            layout: layout()
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

