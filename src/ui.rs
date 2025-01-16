use std::rc::Rc;

use ratatui::{layout::{Constraint, Layout, Rect}, widgets::Paragraph, Frame};
use ratatui_image::StatefulImage;

use crate::{config::{self, Config, LayoutItem}, config_debug, debug_widget, meta::Meta, utils};

pub struct Ui<'a> {
    config: &'a Config
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config
        }
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>, meta: &mut Meta) {
        let main_area = utils::align::get_align(frame, &self.config.align, self.config.width, self.config.height);
        config_debug!(self.config.debug, frame, main_area);

        let areas = self.get_areas(&self.config.layout, &self.config.direction, main_area);

        for (i, item) in self.config.layout.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                config_debug!(self.config.debug, frame, *area);
                self.render_layout(frame, item, area, meta);
            }
        }
    }

    fn render_layout(&self, frame: &mut Frame<'_>, item: &LayoutItem, parent_area: &Rect, meta: &mut Meta) {
        match &item {
            &LayoutItem::Container { width, height, direction, children } => {
                let [area] = Layout::horizontal([Constraint::Length(*width)]).areas(*parent_area);
                let [area] = Layout::vertical([Constraint::Length(*height)]).areas(area);

                let areas = self.get_areas(children, &direction, area);

                for (i, child) in children.iter().enumerate() {
                    if let Some(area) = areas.get(i) {
                        config_debug!(self.config.debug, frame, *area);
                        self.render_layout(frame, child, area, meta);
                    }
                }
            },
            &LayoutItem::CoverArt { width, height } => {
                let [area] = Layout::horizontal([Constraint::Length(*width)]).areas(*parent_area);
                let [area] = Layout::vertical([Constraint::Length(*height)]).areas(area);

                if let Some(cover_art) = meta.cover_art.as_mut() {
                    frame.render_stateful_widget(
                        StatefulImage::default(),
                        area,
                        &mut cover_art.image
                    );
                }
            },
            &LayoutItem::Label { text } => {
                let text = match text {
                    text if text.contains("$title") => &text.replace("$title", &meta.title),
                    text if text.contains("$artists") => &text.replace("$artists", &meta.artists.join(", ")),
                    _ => text
                };

                frame.render_widget(
                    Paragraph::new(text.to_string()),
                    *parent_area
                );
            }
        }
    }

    fn get_areas(&self, items: &Vec<LayoutItem>, direction: &config::Direction, parent_area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(direction.to_dir())
            .constraints(
                items
                    .iter()
                    .map(|child| match child {
                        LayoutItem::Container { width, height, .. } |
                        LayoutItem::CoverArt { width, height } => {
                            match direction {
                                config::Direction::Horizontal => Constraint::Length(*width),
                                config::Direction::Vertical => Constraint::Length(*height)
                            }
                        },
                        LayoutItem::Label { text } => {
                            match direction {
                                config::Direction::Horizontal => Constraint::Length(text.len() as u16),
                                config::Direction::Vertical => Constraint::Length(1)
                            }
                        }
                    })
                    .collect::<Vec<Constraint>>()
            )
            .split(parent_area)
    }
}
