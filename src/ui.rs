use std::collections::HashMap;

use ratatui::{layout::{Position, Rect}, widgets::{Block, Borders, Paragraph, Wrap}, Frame};

use crate::{action::Action, config::Config, config_debug, debug_widget, meta::Meta, utils};

pub struct Ui<'a> {
    config: &'a Config,
    pub buttons: HashMap<String, (Rect, &'a Option<Action>, &'a Option<String>)>
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            buttons: HashMap::new()
        }
    }

    pub fn click(&self, x: u16, y: u16) -> Option<(&Option<Action>, &Option<String>)> {
        for (_, (rect, action, exec)) in self.buttons.iter() {
            if rect.contains(Position::new(x, y)) {
                return Some((
                    action,
                    exec
                ))
            }
        }

        None
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, meta: &mut Meta) {
        let main_area = utils::align::get_align(frame, &self.config.align, self.config.width, self.config.height);

        // Terminal window is too small
        if frame.area().width < self.config.width ||
            frame.area().height < self.config.height {
            frame.render_widget(
                Paragraph::new(format!(
                    "Terminal window is too small. Must have atleast ({}x{}).",
                    self.config.width, self.config.height
                ))
                    .centered()
                    .wrap(Wrap::default())
                    .block(Block::new().borders(Borders::ALL)),
                main_area
            );

            return;
        }

        config_debug!(self.config.debug, frame, main_area);
    }
}
