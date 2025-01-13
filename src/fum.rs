use std::{io::{stdout, Stdout}, time::Duration};

use crossterm::{event::{EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind}, execute};
use mpris::Player;
use ratatui::{prelude::CrosstermBackend, Terminal};
use ratatui_image::picker::Picker;

use crate::{config::Config, meta::Meta, ui::Ui, utils};

pub struct Fum<'a> {
    config: &'a Config,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    ui: Ui<'a>,
    picker: Picker,
    player: Option<Player>,
    meta: Meta,
    redraw: bool,
    exit: bool
}

impl<'a> Fum<'a> {
    pub fn new(config: &'a Config) -> Result<Self, String> {
        let player = utils::player::get_player(&config).ok();

        let picker = Picker::from_query_stdio()
            .expect("Failed to query font size. This terminal might not be supported.");

        let meta = match &player {
            Some(player) => utils::player::get_meta(player, &picker, None)
                .unwrap_or((Meta::default(), false)).0,
            None => Meta::default()
        };

        // Enable mouse capture
        execute!(stdout(), EnableMouseCapture)
            .expect("Failed to enable mouse capture");

        Ok(Self {
            config,
            terminal: ratatui::init(),
            ui: Ui::new(config),
            picker,
            player,
            meta,
            redraw: true, // Draw at startup
            exit: false
        })
    }

    pub fn run(&mut self) {
        while !self.exit {
            if self.redraw {
                self.terminal.draw(|frame| {
                    self.ui.draw(frame, &mut self.meta);
                    self.redraw = false;
                }).expect("Failed to draw frame");
            }

            self.term_events();
            self.update_meta();
        }

        utils::restore();
    }

    fn term_events(&mut self) {
        if crossterm::event::poll(Duration::from_millis(100)).expect("Failed to poll event") {
            let event = crossterm::event::read().expect("Failed to read event");

            match event {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc |
                        KeyCode::Char('q') => {
                            self.exit = true;
                        },
                        KeyCode::Char('p') => self.prev(),
                        KeyCode::Char(' ') => self.play_pause(),
                        KeyCode::Char('n') => self.next(),
                        _ => {}
                    }
                },
                Event::Mouse(mouse) if mouse.kind == MouseEventKind::Down(MouseButton::Left) => {
                    match (mouse.column, mouse.row) {
                        // (x, y) if self.ui.playback_buttons.prev.contains(Position::new(x, y)) => self.prev(),
                        // (x, y) if self.ui.playback_buttons.play_pause.contains(Position::new(x, y)) => self.play_pause(),
                        // (x, y) if self.ui.playback_buttons.next.contains(Position::new(x, y)) => self.next(),
                        _ => {}
                    }
                },
                Event::Resize(_, _) => {
                    self.redraw = true;
                }
                _ => {}
            }
        }
    }

    fn update_meta(&mut self) {
        if let Some(player) = &self.player {
            let (new_meta, changed) = utils::player::get_meta(player, &self.picker, Some(&self.meta))
                .unwrap_or((Meta::default(), true));

            self.meta = new_meta;
            self.redraw = changed;

            return;
        }

        self.player = utils::player::get_player(&self.config).ok();
        self.meta = Meta::default();
    }

    fn prev(&self) {
        if let Some(player) = &self.player {
            player.previous().expect("Failed to prev player");
        }
    }

    fn play_pause(&self) {
        if let Some(player) = &self.player {
            player.play_pause().expect("Failed to play/pause player");
        }
    }

    fn next(&self) {
        if let Some(player) = &self.player {
            player.next().expect("Failed to next player");
        }
    }
}
