use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::Color, widgets::{StatefulWidget, Widget}};
use serde::Deserialize;
use crate::{action::Action, meta::Meta, utils::etc::generate_btn_id};

use super::{button, container, cover_art, empty, label, progress};

fn default_truncate() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Vertical,
    Horizontal
}

impl Default for Direction {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Direction {
    pub fn to_dir(&self) -> ratatui::layout::Direction {
        match self {
            Self::Horizontal => ratatui::layout::Direction::Horizontal,
            Self::Vertical => ratatui::layout::Direction::Vertical
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LabelAlignment {
    Left,
    Center,
    Right
}

impl Default for LabelAlignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerFlex {
    Start,
    Center,
    End,
    #[serde(rename = "space-around")]
    SpaceAround,
    #[serde(rename = "space-between")]
    SpaceBetween
}

impl Default for ContainerFlex {
    fn default() -> Self {
        ContainerFlex::Start
    }
}

impl ContainerFlex {
    pub fn to_flex(&self) -> ratatui::layout::Flex {
        match self {
            Self::Start         => ratatui::layout::Flex::Start,
            Self::Center        => ratatui::layout::Flex::Center,
            Self::End           => ratatui::layout::Flex::End,
            Self::SpaceAround   => ratatui::layout::Flex::SpaceAround,
            Self::SpaceBetween  => ratatui::layout::Flex::SpaceBetween
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProgressOption {
    pub char: char,
    pub bg: Option<Color>,
    pub fg: Option<Color>
}

pub struct FumWidgetState {
    pub meta: Meta
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum FumWidget {
    Container {
        width: Option<u16>,
        height: Option<u16>,
        #[serde(default = "Direction::default")]
        direction: Direction,
        children: Vec<FumWidget>,
        #[serde(default = "ContainerFlex::default")]
        flex: ContainerFlex,
        bg: Option<Color>,
        fg: Option<Color>
    },
    #[serde(rename = "cover-art")]
    CoverArt {
        width: Option<u16>,
        height: Option<u16>,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Label {
        text: String,
        #[serde(default = "LabelAlignment::default")]
        align: LabelAlignment,
        #[serde(default = "default_truncate")]
        truncate: bool,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Button {
        #[serde(default = "generate_btn_id")]
        id: String,
        text: String,
        action: Option<Action>,
        exec: Option<String>,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Progress {
        size: Option<u16>,
        progress: ProgressOption,
        empty: ProgressOption
    },
    Empty {
        size: u16,
        bg: Option<Color>,
        fg: Option<Color>
    }
}

impl StatefulWidget for &FumWidget {
    type State = FumWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized
    {
        match self {
            FumWidget::Container { .. } => container::render(&self, area, buf, state),
            FumWidget::CoverArt { .. } => cover_art::render(&self, area, buf, state),
            FumWidget::Label { .. } => label::render(&self, area, buf, state),
            FumWidget::Button { .. } => button::render(&self, area, buf, state),
            FumWidget::Progress { .. } => progress::render(&self, area, buf, state),
            FumWidget::Empty { .. } => empty::render(&self, area, buf, state)
        }
    }
}

impl FumWidget {
    pub fn get_size(&self, parent_direction: &Direction) -> Constraint {
        match self {
            Self::Container { width, height, direction, .. } => {
                match direction {
                    Direction::Horizontal => width.map(|w| Constraint::Length(w)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => height.map(|h| Constraint::Length(h)).unwrap_or(Constraint::Min(0))
                }
            },
            Self::CoverArt { width, height, .. } => {
                match parent_direction {
                    Direction::Horizontal => width.map(|w| Constraint::Length(w)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => height.map(|h| Constraint::Length(h)).unwrap_or(Constraint::Min(0))
                }
            },
            Self::Label { .. } => {
                match parent_direction {
                    Direction::Horizontal => Constraint::Min(0),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Button { .. } => Constraint::Length(1),
            Self::Progress { size, .. } => {
                match parent_direction {
                    Direction::Horizontal => size.map(|s| Constraint::Length(s)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Empty { size, .. } => Constraint::Length(*size)
        }
    }
}
