use serde::Deserialize;
use crate::utils::etc::generate_btn_id;

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
    },
    #[serde(rename = "cover-art")]
    CoverArt {
        width: Option<u16>,
        height: Option<u16>
    },
    Label {
        text: String,
        #[serde(default = "LabelAlignment::default")]
        align: LabelAlignment,
        #[serde(default = "default_truncate")]
        truncate: bool
    },
    Button {
        #[serde(default = "generate_btn_id")]
        id: String,
        text: String,
        action: Option<String>,
        exec: Option<String>
    },
    Progress {
        size: Option<u16>,
        progress: String,
        empty: String
    },
    Empty {
        size: u16
    }
}
