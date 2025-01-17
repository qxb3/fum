use std::{fs, path::PathBuf};
use serde::Deserialize;

use crate::{utils::generate_btn_id, widget::{ContainerFlex, Direction, FumWidget, LabelAlignment}};

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

fn players() -> Vec<String> { vec!["spotify".to_string()] }
fn use_active_player() -> bool { false }
fn align() -> Align { Align::Center }
fn direction() -> Direction { Direction::Vertical }
fn flex() -> ContainerFlex { ContainerFlex::Start }
fn width() -> u16 { 20 }
fn height() -> u16 { 18 }
fn debug() -> Option<bool> { None }
fn layout() -> Vec<FumWidget> {
    Vec::from([
        FumWidget::CoverArt {
            width: 20,
            height: 10
        },
        FumWidget::Container {
            width: 20,
            height: 10,
            direction: Direction::Vertical,
            flex: ContainerFlex::default(),
            children: Vec::from([
                FumWidget::Label {
                    text: "$title".to_string(),
                    align: LabelAlignment::Center,
                    truncate: true
                },
                FumWidget::Label {
                    text: "$artists".to_string(),
                    align: LabelAlignment::Center,
                    truncate: true
                },
                FumWidget::Container {
                    width: 20,
                    height: 1,
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceAround,
                    children: Vec::from([
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒮".to_string(),
                            action: Some("prev()".to_string()),
                            exec: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "$status_icon".to_string(),
                            action: Some("play_pause()".to_string()),
                            exec: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒭".to_string(),
                            action: Some("next()".to_string()),
                            exec: None
                        }
                    ])
                },
                FumWidget::Progress {
                    size: 20,
                    progress: "󰝤".to_string(),
                    empty: "󰁱".to_string()
                },
                FumWidget::Container {
                    width: 20,
                    height: 1,
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceBetween,
                    children: Vec::from([
                        FumWidget::Label {
                            text: "$position".to_string(),
                            align: LabelAlignment::Left,
                            truncate: false
                        },
                        FumWidget::Label {
                            text: "$length".to_string(),
                            align: LabelAlignment::Right,
                            truncate: false
                        }
                    ])
                }
            ])
        }
    ])
}
