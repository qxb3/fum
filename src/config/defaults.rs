use std::collections::HashMap;

use ratatui::style::Color;

use crate::{action::Action, utils::etc::generate_btn_id, widget::{ContainerFlex, CoverArtResize, Direction, FumWidget, LabelAlignment, ProgressOption}};

use super::{keybind::Keybind, Align};

pub fn players() -> Vec<String> { vec!["spotify".to_string()] }
pub fn use_active_player() -> bool { false }

pub fn align() -> Align { Align::Center }
pub fn direction() -> Direction { Direction::Vertical }
pub fn flex() -> ContainerFlex { ContainerFlex::Start }

pub fn width() -> u16 { 20 }
pub fn height() -> u16 { 18 }

pub fn bg() -> Color { Color::Reset }
pub fn fg() -> Color { Color::Reset }

pub fn cover_art_ascii() -> String { "".to_string() }

pub fn keybinds() -> HashMap<Keybind, Action> {
    HashMap::from([
        (Keybind::Many([Keybind::Esc, Keybind::Char('q')].to_vec()), Action::Quit),
        (Keybind::Char('h'), Action::Prev),
        (Keybind::Char('l'), Action::Next),
        (Keybind::Char(' '), Action::PlayPause)
    ])
}

pub fn layout() -> Vec<FumWidget> {
    Vec::from([
        FumWidget::CoverArt {
            width: None,
            height: Some(10),
            resize: CoverArtResize::Scale,
            bg: None,
            fg: None,
        },
        FumWidget::Container {
            width: None,
            height: Some(10),
            direction: Direction::Vertical,
            flex: ContainerFlex::default(),
            bg: None,
            fg: None,
            children: Vec::from([
                FumWidget::Label {
                    text: "$title".to_string(),
                    align: LabelAlignment::Center,
                    truncate: true,
                    bg: None,
                    fg: None
                },
                FumWidget::Label {
                    text: "$artists".to_string(),
                    align: LabelAlignment::Center,
                    truncate: true,
                    bg: None,
                    fg: None
                },
                FumWidget::Container {
                    width: None,
                    height: Some(1),
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceAround,
                    bg: None,
                    fg: None,
                    children: Vec::from([
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒮".to_string(),
                            action: Some(Action::Prev),
                            exec: None,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "$status_icon".to_string(),
                            action: Some(Action::PlayPause),
                            exec: None,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒭".to_string(),
                            action: Some(Action::Next),
                            exec: None,
                            bg: None,
                            fg: None
                        }
                    ])
                },
                FumWidget::Progress {
                    size: None,
                    progress: ProgressOption {
                        char: '󰝤',
                        bg: None,
                        fg: None
                    },
                    empty: ProgressOption {
                        char: '󰁱',
                        bg: None,
                        fg: None
                    }
                },
                FumWidget::Container {
                    width: None,
                    height: Some(1),
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceBetween,
                    bg: None,
                    fg: None,
                    children: Vec::from([
                        FumWidget::Label {
                            text: "$position".to_string(),
                            align: LabelAlignment::Left,
                            truncate: false,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Label {
                            text: "$length".to_string(),
                            align: LabelAlignment::Right,
                            truncate: false,
                            bg: None,
                            fg: None
                        }
                    ])
                }
            ])
        }
    ])
}
