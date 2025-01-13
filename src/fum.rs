use std::{io::Stdout, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use mpris::Player;
use ratatui::{prelude::CrosstermBackend, Terminal};
use ratatui_image::picker::Picker;

use crate::{config::Config, meta::Meta, term_config::TermConfig, ui::Ui, utils};

pub struct Fum<'a> {
    config: &'a Config,
    term_config: &'a TermConfig,

    terminal: Terminal<CrosstermBackend<Stdout>>,
    picker: Picker,
    player: Option<Player>,
    meta: Meta,
    redraw: bool,
    exit: bool
}

impl<'a> Fum<'a> {
    pub fn new(config: &'a Config, term_config: &'a TermConfig) -> Result<Self, String> {
        let player = utils::player::get_player(&config).ok();

        let picker = Picker::from_query_stdio()
            .expect("Failed to query font size. This terminal might not be supported.");

        let meta = match &player {
            Some(player) => utils::player::get_meta(player, &picker, None)
                .unwrap_or((Meta::default(), false)).0,
            None => Meta::default()
        };

        Ok(Self {
            config,
            term_config,

            terminal: ratatui::init(),
            picker,
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
                    ui.draw(frame, &mut self.meta);
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

        self.meta = Meta::default();
    }
}
