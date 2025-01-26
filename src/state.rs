use std::collections::HashMap;

use ratatui::{layout::Rect, style::Color};

use crate::{action::Action, meta::Meta, widget::Direction};

pub struct FumState {
    pub meta: Meta,
    pub buttons: HashMap<String, (Rect, Option<Action>, Option<String>)>,
    pub parent_direction: Direction,
    pub parent_bg: Color,
    pub parent_fg: Color
}

impl FumState {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            buttons: HashMap::new(),
            parent_direction: Direction::default(),
            parent_bg: Color::Reset,
            parent_fg: Color::Reset
        }
    }
}
