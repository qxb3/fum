use std::{fs, path::PathBuf};
use serde::Deserialize;

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
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Vertical,
    Horizontal
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

    #[serde(default = "width")]
    pub width: u16,

    #[serde(default = "height")]
    pub height: u16,

    #[serde(default = "layout")]
    pub layout: Vec<LayoutItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum LayoutItem {
    Container {
        width: u16,
        height: u16,
        direction: Direction,
        children: Vec<LayoutItem>
    },
    Image {
        width: u16,
        height: u16
    },
    Label {
        text: String
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            players: players(),
            use_active_player: use_active_player(),
            align: align(),
            direction: direction(),
            width: width(),
            height: height(),
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

fn players() -> Vec<String> { vec!["spotify".to_string()] }
fn use_active_player() -> bool { false }
fn align() -> Align { Align::Center }
fn direction() -> Direction { Direction::Vertical }
fn width() -> u16 { 10 }
fn height() -> u16 { 20 }
fn layout() -> Vec<LayoutItem> {
    Vec::from([
        LayoutItem::Image {
            width: 10,
            height: 10
        },
        LayoutItem::Container {
            width: 10,
            height: 10,
            direction: Direction::Vertical,
            children: Vec::from([
                LayoutItem::Label { text: "$title".to_string() },
                LayoutItem::Label { text: "$artists".to_string() }
            ])
        }
    ])
}
