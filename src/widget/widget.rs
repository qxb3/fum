use ratatui::{buffer::Buffer, layout::{Constraint, Rect}, style::Color, widgets::StatefulWidget};
use serde::Deserialize;
use unicode_width::UnicodeWidthStr;
use crate::{action::Action, state::FumState, text::replace_text, utils::etc::generate_id};

use super::{button, container, cover_art, empty, label, progress, volume};

fn default_truncate() -> bool { true }
fn default_border() -> bool { false }
fn default_bold() -> bool { false }

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
#[serde(rename_all = "lowercase")]
pub enum CoverArtResize {
    Fit,
    Crop,
    Scale
}

impl Default for CoverArtResize {
    fn default() -> Self {
        Self::Scale
    }
}

impl CoverArtResize {
    pub fn to_resize(&self) -> ratatui_image::Resize {
        match self {
            Self::Fit       => ratatui_image::Resize::Fit(Some(ratatui_image::FilterType::CatmullRom)),
            Self::Crop      => ratatui_image::Resize::Crop(None),
            Self::Scale     => ratatui_image::Resize::Scale(Some(ratatui_image::FilterType::CatmullRom))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProgressOption {
    pub char: char,
    pub bg: Option<Color>,
    pub fg: Option<Color>
}

#[derive(Debug, Clone, Deserialize)]
pub struct VolumeOption {
    pub char: char,
    pub bg: Option<Color>,
    pub fg: Option<Color>
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
        #[serde(default = "default_border")]
        border: bool,
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
        #[serde(default = "CoverArtResize::default")]
        resize: CoverArtResize,
        #[serde(default = "default_border")]
        border: bool,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Label {
        text: String,
        #[serde(default = "Direction::default")]
        direction: Direction,
        #[serde(default = "LabelAlignment::default")]
        align: LabelAlignment,
        #[serde(default = "default_truncate")]
        truncate: bool,
        #[serde(default = "default_bold")]
        bold: bool,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Button {
        #[serde(default = "generate_id")]
        id: String,
        text: String,
        action: Option<Action>,
        exec: Option<String>,
        #[serde(default = "Direction::default")]
        direction: Direction,
        #[serde(default = "default_bold")]
        bold: bool,
        bg: Option<Color>,
        fg: Option<Color>
    },
    Progress {
        #[serde(default = "generate_id")]
        id: String,
        size: Option<u16>,
        #[serde(default = "Direction::default")]
        direction: Direction,
        progress: ProgressOption,
        empty: ProgressOption
    },
    Volume {
        #[serde(default = "generate_id")]
        id: String,
        size: Option<u16>,
        #[serde(default = "Direction::default")]
        direction: Direction,
        volume: VolumeOption,
        empty: VolumeOption
    },
    Empty {
        size: u16,
        bg: Option<Color>,
        fg: Option<Color>
    }
}

impl StatefulWidget for &FumWidget {
    type State = FumState;

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
            FumWidget::Volume { .. } => volume::render(&self, area, buf, state),
            FumWidget::Empty { .. } => empty::render(&self, area, buf, state)
        }
    }
}

impl FumWidget {
    pub fn get_size(&self, state: &mut FumState) -> Constraint {
        match self {
            Self::Container { width, height, direction, .. } => {
                match direction {
                    Direction::Horizontal => width.map(|w| Constraint::Length(w)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => height.map(|h| Constraint::Length(h)).unwrap_or(Constraint::Min(0))
                }
            },
            Self::CoverArt { width, height, .. } => {
                match &state.parent_direction {
                    Direction::Horizontal => width.map(|w| Constraint::Length(w)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => height.map(|h| Constraint::Length(h)).unwrap_or(Constraint::Min(0))
                }
            },
            Self::Label { direction, .. } => {
                match direction {
                    Direction::Horizontal => Constraint::Min(0),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Button { direction, text, .. } => {
                match direction {
                    Direction::Horizontal => Constraint::Length(UnicodeWidthStr::width(replace_text(text, state).as_str()) as u16),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Progress { size, direction, .. } => {
                match direction {
                    Direction::Horizontal => size.map(|s| Constraint::Length(s)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Volume { size, direction, .. } => {
                match direction {
                    Direction::Horizontal => size.map(|s| Constraint::Length(s)).unwrap_or(Constraint::Min(0)),
                    Direction::Vertical => Constraint::Length(1)
                }
            },
            Self::Empty { size, .. } => Constraint::Length(*size)
        }
    }
}
