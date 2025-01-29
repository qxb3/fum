use std::{collections::HashMap, fs, path::PathBuf};
use expanduser::expanduser;
use ratatui::style::Color;
use serde::Deserialize;

use crate::{action::Action, fum::FumResult, widget::{ContainerFlex, Direction, FumWidget}};

use super::{defaults::{align, bg, cover_art_ascii, direction, fg, flex, height, keybinds, layout, players, use_active_player, width}, keybind::Keybind};

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

    #[serde(default = "keybinds")]
    pub keybinds: HashMap<Keybind, Action>,

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

    #[serde(default = "bg")]
    pub bg: Color,

    #[serde(default = "fg")]
    pub fg: Color,

    #[serde(default = "cover_art_ascii")]
    pub cover_art_ascii: String,

    #[serde(default = "layout")]
    pub layout: Vec<FumWidget>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            use_active_player: use_active_player(),
            keybinds: keybinds(),
            align: align(),
            direction: direction(),
            flex: flex(),
            width: width(),
            height: height(),
            bg: bg(),
            fg: fg(),
            cover_art_ascii: cover_art_ascii(),
            layout: layout()
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> FumResult<Self> {
        match fs::read_to_string(path) {
            Ok(config_file) => {
                let mut config: Config = serde_json::from_str(&config_file)
                    .map_err(|err| format!("Failed to parse config: {err}"))?;

                // Get expanded path of cover_art_ascii
                let cover_art_ascii_path = expanduser(&config.cover_art_ascii)
                    .map_err(|err| format!("Failed to expand path of cover_art_ascii: {err}"))?;

                // Load the cover_art_ascii
                match fs::read_to_string(cover_art_ascii_path) {
                    Ok(cover_art_ascii) => config.cover_art_ascii = cover_art_ascii,
                    Err(_) => config.cover_art_ascii = "".to_string()
                }

                Ok(config)
            },
            Err(_) => Ok(Config::default())
        }
    }
}

