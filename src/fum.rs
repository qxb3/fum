use std::{io::Stdout, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use mpris::Player;
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{config::Config, meta::Meta, term_config::TermConfig, ui::Ui, utils};

pub struct Fum<'a> {
    config: &'a Config,
    term_config: &'a TermConfig,

    terminal: Terminal<CrosstermBackend<Stdout>>,
    player: Option<Player>,
    meta: Meta,
    redraw: bool,
    exit: bool
}

impl<'a> Fum<'a> {
    pub fn new(config: &'a Config, term_config: &'a TermConfig) -> Result<Self, String> {
        let player = utils::player::get_player(&config).ok();

        let meta = match &player {
            Some(player) => utils::player::get_meta(player, None)
                .unwrap_or((Meta::default(), false)).0,
            None => Meta::default()
        };

        Ok(Self {
            config,
            term_config,

            terminal: ratatui::init(),
            player,
            meta,
            redraw: true, // Draw at startup
            exit: false
        })
    }

    pub fn run(&mut self) {
        let mut ui = Ui::new(&self.config, &self.term_config);

        while !self.exit {
            if self.redraw {
                self.terminal.draw(|frame| {
                    ui.draw(frame, &self.meta);
                    self.redraw = false;
                }).expect("Failed to draw frame");
            }

            self.term_events();
            self.update_meta();
        }

        ratatui::restore();
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
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    fn update_meta(&mut self) {
        if let Some(player) = &self.player {
            let (new_meta, changed) = utils::player::get_meta(player, Some(&self.meta))
                .expect("Failed to get player metadata");

            self.meta = new_meta;
            self.redraw = changed;

            return;
        }

        self.meta = Meta::default();
    }
}
