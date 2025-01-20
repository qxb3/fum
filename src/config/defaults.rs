use std::collections::HashMap;

use crate::{action::Action, utils::etc::generate_btn_id, widget::{ContainerFlex, Direction, FumWidget, LabelAlignment}};

use super::{keybind::Keybind, Align};

pub fn players() -> Vec<String> { vec!["spotify".to_string()] }
pub fn use_active_player() -> bool { false }
pub fn align() -> Align { Align::Center }
pub fn direction() -> Direction { Direction::Vertical }
pub fn flex() -> ContainerFlex { ContainerFlex::Start }
pub fn width() -> u16 { 20 }
pub fn height() -> u16 { 18 }
pub fn debug() -> Option<bool> { None }
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
            height: Some(10)
        },
        FumWidget::Container {
            width: None,
            height: Some(10),
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
                    width: None,
                    height: Some(1),
                    direction: Direction::Horizontal,
                    flex: ContainerFlex::SpaceAround,
                    children: Vec::from([
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒮".to_string(),
                            action: Some(Action::Prev),
                            exec: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "$status_icon".to_string(),
                            action: Some(Action::PlayPause),
                            exec: None
                        },
                        FumWidget::Button {
                            id: generate_btn_id(),
                            text: "󰒭".to_string(),
                            action: Some(Action::Next),
                            exec: None
                        }
                    ])
                },
                FumWidget::Progress {
                    size: None,
                    progress: "󰝤".to_string(),
                    empty: "󰁱".to_string()
                },
                FumWidget::Container {
                    width: None,
                    height: Some(1),
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
