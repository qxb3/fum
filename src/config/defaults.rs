use std::collections::HashMap;

use ratatui::style::Color;

use crate::{action::Action, utils::etc::generate_id, widget::{ContainerFlex, CoverArtResize, Direction, FumWidget, LabelAlignment, ProgressOption}};

use super::{keybind::Keybind, Align};

pub fn players() -> Vec<String> { vec!["spotify".to_string()] }
pub fn use_active_player() -> bool { false }
pub fn fps() -> i32 { 10 }

pub fn align() -> Align { Align::Center }
pub fn direction() -> Direction { Direction::Vertical }
pub fn flex() -> ContainerFlex { ContainerFlex::Start }

pub fn width() -> u16 { 19 }
pub fn height() -> u16 { 15 }

pub fn border() -> bool { false }

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
            height: None,
            border: false,
            resize: CoverArtResize::Scale,
            bg: None,
            fg: None,
        },
        FumWidget::Empty {
            size: 1,
            bg: None,
            fg: None
        },
        FumWidget::Container {
            width: None,
            height: None,
            direction: Direction::Vertical,
            border: false,
            flex: ContainerFlex::default(),
            bg: None,
            fg: None,
            children: Vec::from([
                FumWidget::Label {
                    text: "$title".to_string(),
                    direction: Direction::default(),
                    align: LabelAlignment::Center,
                    truncate: true,
                    bold: false,
                    bg: None,
                    fg: None
                },
                FumWidget::Label {
                    text: "$artists".to_string(),
                    direction: Direction::default(),
                    align: LabelAlignment::Center,
                    truncate: true,
                    bold: false,
                    bg: None,
                    fg: None
                },
                FumWidget::Empty {
                    size: 1,
                    bg: None,
                    fg: None
                },
                FumWidget::Container {
                    width: None,
                    height: Some(1),
                    direction: Direction::Horizontal,
                    border: false,
                    flex: ContainerFlex::SpaceAround,
                    bg: None,
                    fg: None,
                    children: Vec::from([
                        FumWidget::Button {
                            id: generate_id(),
                            text: "󰒮".to_string(),
                            direction: Direction::default(),
                            action: Some(Action::Prev),
                            exec: None,
                            bold: false,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Button {
                            id: generate_id(),
                            text: "$status_icon".to_string(),
                            direction: Direction::default(),
                            action: Some(Action::PlayPause),
                            exec: None,
                            bold: false,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Button {
                            id: generate_id(),
                            text: "󰒭".to_string(),
                            direction: Direction::default(),
                            action: Some(Action::Next),
                            exec: None,
                            bold: false,
                            bg: None,
                            fg: None
                        }
                    ])
                },
                FumWidget::Progress {
                    id: generate_id(),
                    size: None,
                    direction: Direction::Horizontal,
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
                    border: false,
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceBetween,
                    bg: None,
                    fg: None,
                    children: Vec::from([
                        FumWidget::Label {
                            text: "$position".to_string(),
                            direction: Direction::default(),
                            align: LabelAlignment::Left,
                            truncate: false,
                            bold: false,
                            bg: None,
                            fg: None
                        },
                        FumWidget::Label {
                            text: "$length".to_string(),
                            direction: Direction::default(),
                            align: LabelAlignment::Right,
                            truncate: false,
                            bold: false,
                            bg: None,
                            fg: None
                        }
                    ])
                }
            ])
        }
    ])
}
