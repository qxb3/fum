use ratatui::Frame;

use crate::{config::Config, meta::Meta};

pub struct Ui<'a> {
    config: &'a Config
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config
        }
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, meta: &mut Meta) {}
}
