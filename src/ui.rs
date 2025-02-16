use std::collections::HashMap;

use ratatui::{layout::{Constraint, Layout, Margin, Position, Rect}, style::Stylize, widgets::{Block, Borders, Paragraph, Wrap}, Frame};

use crate::{action::Action, config::Config, get_border, state::FumState, utils, widget::{Direction, SliderSource}};

pub struct Ui<'a> {
    config: &'a Config,
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
        }
    }

    pub fn click(
        &self,
        x: u16,
        y: u16,
        buttons: &'a HashMap<String, (Rect, Option<Action>, Option<Action>, Option<String>)>
    ) -> Option<(&'a Option<Action>, &'a Option<Action>, &'a Option<String>)> {
        for (_, (rect, action, action_secondary, exec)) in buttons.iter() {
            if rect.contains(Position::new(x, y)) {
                return Some((
                    action,
                    action_secondary,
                    exec
                ))
            }
        }

        None
    }

    pub fn drag(
        &self,
        start_drag: &Position,
        sliders: &HashMap<String, (Rect, Direction, SliderSource)>
    ) -> Option<(Rect, Direction, SliderSource)> {
        for (_, (rect, direction, widget)) in sliders.iter() {
            if rect.contains(*start_drag) {
                return Some((*rect, direction.to_owned(), *widget));
            }
        }

        None
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, state: &mut FumState) {
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

        // Sets the state parents state
        state.parent_direction = self.config.direction.to_owned();
        state.parent_bg = self.config.bg;
        state.parent_fg = self.config.fg;

        // Also pass in the cover_art_ascii on state
        state.cover_art_ascii = self.config.cover_art_ascii.to_owned();

        let areas = Layout::default()
            .direction(self.config.direction.to_dir())
            .flex(self.config.flex.to_flex())
            .constraints(
                self.config.layout
                    .iter()
                    .map(|child| child.get_size(state))
                    .collect::<Vec<Constraint>>()
            )
            .split(main_area);

        // Whether to render border
        let border = get_border!(&self.config.border);

        // Render background
        frame.render_widget(
            Block::new()
                .borders(border)
                .bg(state.parent_bg)
                .fg(state.parent_fg),
            main_area
        );

        for (i, widget) in self.config.layout.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                let [horizontal_padding, vertical_padding] = &self.config.padding;
                frame.render_stateful_widget(widget, area.inner(Margin::new(*horizontal_padding, *vertical_padding)), state);
            }
        }
    }
}
