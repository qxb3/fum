use std::rc::Rc;

use ratatui::{layout::{Constraint, Layout, Rect}, widgets::Paragraph, Frame};
use ratatui_image::StatefulImage;

use crate::{config::{self, Config, ContainerFlex, FumWidget, LabelAlignment}, config_debug, debug_widget, meta::Meta, utils};

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

        let areas = self.get_areas(
            &self.config.layout,
            &self.config.direction,
            &self.config.flex,
            main_area
        );

        for (i, item) in self.config.layout.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                config_debug!(self.config.debug, frame, *area);
                self.render_layout(frame, item, area, meta);
            }
        }
    }

    fn render_layout(&self, frame: &mut Frame<'_>, item: &FumWidget, parent_area: &Rect, meta: &mut Meta) {
        match &item {
            &FumWidget::Container { width, height, direction, flex, children } => {
                let [area] = Layout::horizontal([Constraint::Length(*width)]).areas(*parent_area);
                let [area] = Layout::vertical([Constraint::Length(*height)]).areas(area);

                let flex = match flex {
                    Some(flex) => flex,
                    None => &ContainerFlex::Start
                };

                let areas = self.get_areas(
                    children,
                    &direction,
                    flex,
                    area
                );

                for (i, child) in children.iter().enumerate() {
                    if let Some(area) = areas.get(i) {
                        config_debug!(self.config.debug, frame, *area);
                        self.render_layout(frame, child, area, meta);
                    }
                }
            },
            &FumWidget::CoverArt { width, height } => {
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
            &FumWidget::Label { text, align } => {
                let text = self.replace_text(text, meta).to_string();

                let widget = match align {
                    Some(align) => match align {
                        LabelAlignment::Left => Paragraph::new(text).left_aligned(),
                        LabelAlignment::Center => Paragraph::new(text).centered(),
                        LabelAlignment::Right => Paragraph::new(text).right_aligned(),
                    },
                    None => Paragraph::new(text)
                };

                frame.render_widget(
                    widget,
                    *parent_area
                );
            }
            &FumWidget::Button { text, action, exec } => {
                let text = self.replace_text(text, meta).to_string();

                frame.render_widget(
                    Paragraph::new(text),
                    *parent_area
                );
            }
        }
    }

    fn replace_text(&self, text: &String, meta: &mut Meta) -> String {
        match text {
            text if text.contains("$title") => text.replace("$title", &meta.title),
            text if text.contains("$artists") => text.replace("$artists", &meta.artists.join(", ")),
            text if text.contains("$status_icon") => text.replace("$status_icon", &meta.status_icon),
            _ => text.to_string()
        }
    }

    fn get_areas(&self, items: &Vec<FumWidget>, direction: &config::Direction, flex: &ContainerFlex, parent_area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(direction.to_dir())
            .flex(flex.to_flex())
            .constraints(
                items
                    .iter()
                    .map(|child| match child {
                        FumWidget::Container { width, height, .. } |
                        FumWidget::CoverArt { width, height } => {
                            match direction {
                                config::Direction::Horizontal => Constraint::Length(*width),
                                config::Direction::Vertical => Constraint::Length(*height)
                            }
                        },
                        FumWidget::Label { .. } => {
                            match direction {
                                config::Direction::Horizontal => Constraint::Min(0),
                                config::Direction::Vertical => Constraint::Length(1)
                            }
                        },
                        FumWidget::Button { .. } => {
                            Constraint::Length(1)
                        }
                    })
                    .collect::<Vec<Constraint>>()
            )
            .split(parent_area)
    }
}
