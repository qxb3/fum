use std::collections::HashMap;

use ratatui::{layout::{Constraint, Layout, Position, Rect}, widgets::{Block, Borders, Paragraph, Wrap}, Frame};

use crate::{action::Action, config::Config, config_debug, debug_widget, meta::Meta, utils, widget::FumWidgetState};

pub struct Ui<'a> {
    config: &'a Config,
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
        }
    }

    pub fn click(&self, x: u16, y: u16, buttons: &'a HashMap<String, (Rect, Option<Action>, Option<String>)>) -> Option<(&'a Option<Action>, &'a Option<String>)> {
        for (_, (rect, action, exec)) in buttons.iter() {
            if rect.contains(Position::new(x, y)) {
                return Some((
                    action,
                    exec
                ))
            }
        }

        None
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, widget_state: &mut FumWidgetState) {
        let main_area = utils::align::get_align(frame, &self.config.align, self.config.width, self.config.height);

        // Terminal window is too small
        if &frame.area().width < &self.config.width ||
            &frame.area().height < &self.config.height {
            frame.render_widget(
                Paragraph::new(format!(
                    "Terminal window is too small. Must have atleast ({}x{}).",
                    &self.config.width, &self.config.height
                ))
                    .centered()
                    .wrap(Wrap::default())
                    .block(Block::new().borders(Borders::ALL)),
                main_area
            );

            return;
        }

        let areas = Layout::default()
            .direction(self.config.direction.to_dir())
            .flex(self.config.flex.to_flex())
            .constraints(
                self.config.layout
                    .iter()
                    .map(|child| child.get_size(&self.config.direction))
                    .collect::<Vec<Constraint>>()
            )
            .split(main_area);

        for (i, widget) in self.config.layout.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                frame.render_stateful_widget(widget, *area, widget_state);
            }
        }
    }
}
