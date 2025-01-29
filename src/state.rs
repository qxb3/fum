use std::collections::HashMap;

use ratatui::{layout::Rect, style::Color};

use crate::{action::Action, meta::Meta, widget::Direction};

pub struct FumState {
    pub meta: Meta,
    pub cover_art_ascii: String,
    pub buttons: HashMap<String, (Rect, Option<Action>, Option<String>)>,
    pub vars: HashMap<String, String>,
    pub parent_direction: Direction,
    pub parent_bg: Color,
    pub parent_fg: Color
}

impl FumState {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            cover_art_ascii: String::new(),
            buttons: HashMap::new(),
            vars: HashMap::new(),
            parent_direction: Direction::default(),
            parent_bg: Color::Reset,
            parent_fg: Color::Reset
        }
    }
}
