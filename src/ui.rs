use core::f64;
use std::{collections::HashMap, rc::Rc};

use ratatui::{layout::{Constraint, Layout, Rect}, text::Text, widgets::Paragraph, Frame};
use ratatui_image::StatefulImage;

use crate::{config::Config, config_debug, debug_widget, meta::Meta, utils, widget::{self, ContainerFlex, FumWidget, LabelAlignment}};

pub struct Ui<'a> {
    config: &'a Config,
    pub buttons: HashMap<String, (Rect, String, Option<String>)>
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            buttons: HashMap::new()
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

        for (i, widget) in self.config.layout.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                config_debug!(self.config.debug, frame, *area);
                self.render_layout(frame, widget, area, meta);
            }
        }
    }

    fn render_layout(&mut self, frame: &mut Frame<'_>, widget: &FumWidget, parent_area: &Rect, meta: &mut Meta) {
        match &widget {
            FumWidget::Container { width, height, direction, flex, children } => {
                let [area] = Layout::horizontal([Constraint::Length(*width)]).areas(*parent_area);
                let [area] = Layout::vertical([Constraint::Length(*height)]).areas(area);

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
            FumWidget::CoverArt { width, height } => {
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
            FumWidget::Label { text, align } => {
                let text = self.replace_text(text, meta).to_string();

                let widget = match align {
                    LabelAlignment::Left => Paragraph::new(text).left_aligned(),
                    LabelAlignment::Center => Paragraph::new(text).centered(),
                    LabelAlignment::Right => Paragraph::new(text).right_aligned(),
                };

                frame.render_widget(
                    widget,
                    *parent_area
                );
            }
            FumWidget::Button { id, text, action, exec } => {
                let text = self.replace_text(text, meta).to_string();

                self.buttons.insert(
                    id.to_string(),
                    (*parent_area, action.to_string(), exec.to_owned())
                );

                frame.render_widget(
                    Paragraph::new(text),
                    *parent_area
                );
            },
            FumWidget::Progress { progress: progress_char, empty: empty_char, .. } => {
                if meta.length.as_secs() > 0 {
                    let ratio = meta.position.as_secs() as f64 / meta.length.as_secs() as f64;

                    let filled = (ratio * parent_area.width as f64).round();
                    let empty = parent_area.width.saturating_sub(filled as u16);

                    let filled_bar = progress_char.repeat(filled as usize);
                    let empty_bar = empty_char.repeat(empty.into());

                    frame.render_widget(Text::from(format!("{filled_bar}{empty_bar}")), *parent_area);
                } else {
                    frame.render_widget(Text::from(empty_char.repeat(parent_area.width.into())), *parent_area);
                }
            },
            FumWidget::Empty { .. } => {}
        }
    }

    fn replace_text(&self, text: &String, meta: &mut Meta) -> String {
        match text {
            text if text.contains("$title") => text.replace("$title", &meta.title),
            text if text.contains("$artists") => text.replace("$artists", &meta.artists.join(", ")),
            text if text.contains("$status_icon") => text.replace("$status_icon", &meta.status_icon),
            text if text.contains("$position") => text.replace("$position", &utils::format_duration(meta.position)),
            text if text.contains("$length") => text.replace("$length", &utils::format_duration(meta.length)),
            _ => text.to_string()
        }
    }

    fn get_areas(&self, widgets: &Vec<FumWidget>, direction: &widget::Direction, flex: &ContainerFlex, parent_area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(direction.to_dir())
            .flex(flex.to_flex())
            .constraints(
                widgets
                    .iter()
                    .map(|child| match child {
                        FumWidget::Container { width, height, .. } |
                        FumWidget::CoverArt { width, height } => {
                            match direction {
                                widget::Direction::Horizontal => Constraint::Length(*width),
                                widget::Direction::Vertical => Constraint::Length(*height)
                            }
                        },
                        FumWidget::Label { .. } => {
                            match direction {
                                widget::Direction::Horizontal => Constraint::Min(0),
                                widget::Direction::Vertical => Constraint::Length(1)
                            }
                        },
                        FumWidget::Button { .. } => {
                            Constraint::Length(1)
                        },
                        FumWidget::Progress { size, .. } => {
                            match direction {
                                widget::Direction::Horizontal => Constraint::Length(*size),
                                widget::Direction::Vertical => Constraint::Length(1)
                            }
                        },
                        FumWidget::Empty { size } => {
                            Constraint::Length(*size)
                        }
                    })
                    .collect::<Vec<Constraint>>()
            )
            .split(parent_area)
    }
}
