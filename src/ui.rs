use std::time::Duration;

use ratatui::{layout::{Constraint, Flex, Layout, Rect}, text::Text, widgets::{Block, Borders, Paragraph, Wrap}, Frame};
use ratatui_image::{picker::Picker, StatefulImage};
use crate::{config::Config, term_config::TermConfig, utils, meta::Meta};

pub struct PlaybackButtons {
    pub prev: Rect,
    pub toggle: Rect,
    pub next: Rect
}

impl Default for PlaybackButtons {
    fn default() -> Self {
        Self {
            prev: Rect::default(),
            toggle: Rect::default(),
            next:Rect::default()
        }
    }
}

pub struct Ui<'a> {
    pub picker: Picker,
    pub playback_buttons: PlaybackButtons,
    config: &'a Config,
    term_config: &'a TermConfig
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config, term_config: &'a TermConfig) -> Self {
        let picker = Picker::from_query_stdio()
            .expect("Failed to query font size. This terminal might not be supported.");

        let playback_buttons = PlaybackButtons::default();

        Self {
            picker,
            playback_buttons,
            config,
            term_config
        }
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame<'_>,
        meta: &Meta
    ) {
        let area = utils::align::get_align(
            &self.config.align,
            frame,
            self.term_config.width,
            self.term_config.height
        );

        // Terminal window is too small
        if frame.area().width < self.term_config.width ||
            frame.area().height < self.term_config.height {
            frame.render_widget(
                Paragraph::new(format!(
                    "Terminal window is too small. Must have atleast ({}x{}).",
                    self.term_config.width, self.term_config.height
                ))
                    .centered()
                    .wrap(Wrap::default())
                    .block(Block::new().borders(Borders::ALL)),
                area
            );

            return;
        }

        let (image_area, meta_area) = utils::layout::get_layout(
            &self.config.layout,
            area
        );

        let (
            title_area,
            artists_area,
            buttons_area,
            progress_area
        ) = match self.config.layout.as_str() {
            "bottom-to-top" => {
                let [progress_area, _, buttons_area, _, artists_area, title_area] = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]).areas(meta_area);

                (title_area, artists_area, buttons_area, progress_area)
            },
            _ => {
                let [title_area, artists_area, _, buttons_area, _, progress_area] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(2),
                ]).areas(meta_area);

                (title_area, artists_area, buttons_area, progress_area)
            }
        };

        frame.render_stateful_widget(
            StatefulImage::default(),
            image_area,
            &mut self.picker.new_resize_protocol(meta.cover_art.clone())
        );

        if !self.config.hidden.contains(&"title".to_string()) {
            frame.render_widget(
                Text::from(utils::truncate(&meta.title, 14).as_str())
                    .centered(),
                title_area
            );
        }

        if !self.config.hidden.contains(&"artists".to_string()) {
            frame.render_widget(
                Text::from(utils::truncate(&meta.artists.join(", "), 14).as_str())
                    .centered(),
                artists_area
            );
        }

        if !self.config.hidden.contains(&"buttons".to_string()) {
            self.render_buttons(frame, buttons_area, &meta);
        }

        self.render_progress(frame, progress_area, &meta);
    }

    fn render_buttons(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        meta: &Meta
    ) {
        let [prev, _, toggle, _, next] = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1)
        ])
            .flex(Flex::Center)
            .areas(area);

        self.playback_buttons.prev = prev;
        self.playback_buttons.toggle = toggle;
        self.playback_buttons.next = next;

        frame.render_widget(
            Text::from("󰒮"),
            prev
        );

        frame.render_widget(
            Text::from(utils::player::get_status_icon(&meta.status)),
            toggle
        );

        frame.render_widget(
            Text::from("󰒭"),
            next
        );
    }

    fn render_progress(
        &mut self,
        frame: &mut Frame<'_>,
        area: Rect,
        meta: &Meta
    ) {
        let (
            progress_area,
            progress_text_area
        ) = match self.config.layout.as_str() {
            "bottom-to-top" => {
                let [progress_text_area, progress_area] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1)
                ]).areas(area);

                (progress_area, progress_text_area)
            },
            _ => {
                let [progress_area, progress_text_area] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(1)
                ]).areas(area);

                (progress_area, progress_text_area)
            }
        };

        let [current_pos_area, length_area] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Min(0)
        ])
            .flex(Flex::SpaceBetween)
            .areas(progress_text_area);

        if !self.config.hidden.contains(&"progress-bar".to_string()) {
            if meta.length.as_secs() > 0 {
                let ratio = meta.position.as_secs() as f64 / meta.length.as_secs() as f64;
                let filled = (ratio * progress_area.width as f64).round();
                let empty = progress_area.width.saturating_sub(filled as u16);
                let filled_bar = self.config.progress.to_string().repeat(filled as usize);
                let empty_bar = self.config.empty.to_string().repeat(empty.into());

                frame.render_widget(Text::from(format!("{filled_bar}{empty_bar}")), progress_area);
            } else {
                frame.render_widget(Text::from(self.config.empty.to_string().repeat(progress_area.width.into())), progress_area);
            }
        }

        if !self.config.hidden.contains(&"progress-text".to_string()) {
            frame.render_widget(
                Text::from(format!(
                    "{}",
                    if meta.position.as_secs() >= 3600 {
                        format!(
                            "{}:{:02}:{:02}",
                            meta.position.as_secs() / 3600,
                            (meta.position.as_secs() % 3600) / 60,
                            meta.position.as_secs() % 60
                        )
                    } else {
                        format!(
                            "{}:{:02}",
                            meta.position.as_secs() / 60,
                            meta.position.as_secs() % 60
                        )
                    }
                ))
                    .left_aligned(),
                current_pos_area,
            );

            frame.render_widget(
                Text::from(format!(
                    "{}",
                    if meta.length.as_secs() >= 3600 {
                        format!(
                            "{}:{:02}:{:02}",
                            meta.length.as_secs() / 3600,
                            (meta.length.as_secs() % 3600) / 60,
                            meta.length.as_secs() % 60
                        )
                    } else {
                        format!(
                            "{}:{:02}",
                            meta.length.as_secs() / 60,
                            meta.length.as_secs() % 60
                        )
                    }
                ))
                    .right_aligned(),
                length_area,
            );
        }
    }
}
