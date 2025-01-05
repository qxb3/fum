use std::time::Duration;

use ratatui::{layout::{Constraint, Flex, Layout, Rect}, text::Text, widgets::{Block, Borders, Paragraph, Wrap}, Frame};
use ratatui_image::{picker::Picker, StatefulImage};
use crate::{config::Config, utils, Meta};

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
    config: &'a Config
}

impl<'a> Ui<'a> {
    pub fn new(config: &'a Config) -> Result<Self, &'a str> {
        let picker = Picker::from_query_stdio()
            .map_err(|_| "Failed to query font size. This terminal might not be supported.")?;

        let playback_buttons = PlaybackButtons::default();

        Ok(Self {
            picker,
            playback_buttons,
            config
        })
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame<'_>,
        meta: &Meta,
        current_progress: &Duration
    ) {
        let [area] = Layout::horizontal([Constraint::Length(self.config.width)])
            .flex(Flex::Center)
            .areas(frame.area());

        let [area] = Layout::vertical([Constraint::Length(self.config.height)])
            .flex(Flex::Center)
            .areas(area);

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
                area
            );

            return;
        }

        let [top, bottom] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        let [title, artist, _, controls, _, progress, pos] = Layout::vertical([
            Constraint::Length(1); 7
        ])
            .flex(Flex::Center)
            .areas(bottom);

        let [prev, toggle, next] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
            .flex(Flex::Center)
            .areas(controls);

        self.playback_buttons.prev = prev;
        self.playback_buttons.toggle = toggle;
        self.playback_buttons.next = next;

        let [pos_current, _, length] = Layout::horizontal([
            Constraint::Length(4),
            Constraint::Fill(8),
            Constraint::Length(4),
        ])
            .spacing(1)
            .areas(pos);

        frame.render_stateful_widget(
            StatefulImage::default(),
            top,
            &mut self.picker.new_resize_protocol(meta.cover_art.clone())
        );

        frame.render_widget(
            Text::from(
                utils::truncate(&meta.title, 12).as_str()
            ).centered(),
            title
        );

        frame.render_widget(
            Text::from(
                utils::truncate(&meta.artists.join(", "), 12).as_str()
            ).centered(),
            artist
        );

        frame.render_widget(Text::from("󰒮").centered(), self.playback_buttons.prev);
        frame.render_widget(
            Text::from(
                utils::player::get_status_icon(&meta.status)
            ).centered(),
            self.playback_buttons.toggle
        );
        frame.render_widget(Text::from("󰒭").centered(), self.playback_buttons.next);

        if meta.length.as_secs() > 0{
            let ratio = current_progress.as_secs() as f64 / meta.length.as_secs() as f64;
            let filled = (ratio * self.config.width as f64).round();
            let empty = self.config.width.saturating_sub(filled as u16);
            let filled_bar = "󰝤".repeat(filled as usize);
            let empty_bar = "󰁱".repeat(empty.into());

            frame.render_widget(Text::from(format!("{filled_bar}{empty_bar}")), progress);
        } else {
            frame.render_widget(Text::from("󰁱".repeat(self.config.width.into())), progress);
        }

        frame.render_widget(
            Text::from(format!("{}:{:02}", current_progress.as_secs() / 60, current_progress.as_secs() % 60)).left_aligned(),
            pos_current
        );

        frame.render_widget(
            Text::from(format!("{}:{:02}", meta.length.as_secs() / 60, meta.length.as_secs() % 60)).right_aligned(),
            length
        );
    }
}
