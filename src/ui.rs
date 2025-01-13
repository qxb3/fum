use ratatui::Frame;

use crate::{config::Config, term_config::TermConfig, meta::Meta};

pub struct Ui<'a> {
    config: &'a Config,
    term_config: &'a TermConfig
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config, term_config: &'a TermConfig) -> Self {
        Self {
            config,
            term_config
        }
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, meta: &mut Meta) {}
}
