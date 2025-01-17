use std::{io::{stdout, Stdout}, process::Command, time::Duration};

use crossterm::{event::{EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind}, execute};
use mpris::{LoopStatus, Player};
use ratatui::{layout::Position, prelude::CrosstermBackend, Terminal};
use ratatui_image::picker::Picker;

use crate::{config::Config, meta::Meta, ui::Ui, utils};

pub struct Fum<'a> {
    config: &'a Config,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    ui: Ui<'a>,
    picker: Picker,
    player: Option<Player>,
    meta: Meta<'a>,
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

        utils::terminal::restore();
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
                    for (rect, action, exec) in self.ui.buttons.values() {
                        if rect.contains(Position::new(mouse.column, mouse.row)) {
                            // Execute action
                            if let Some(action) = action {
                                match action.as_str() {
                                    "stop()" => self.stop(),
                                    "play()" => self.play(),
                                    "pause()" => self.pause(),

                                    "prev()" => self.prev(),
                                    "play_pause()" => self.play_pause(),
                                    "next()" => self.next(),

                                    "shuffle_off()" => self.shuffle("off"),
                                    "shuffle_toggle()" => self.shuffle("toggle"),
                                    "shuffle_on()" => self.shuffle("on"),

                                    "loop_none()" => self.r#loop("none"),
                                    "loop_track()" => self.r#loop("track"),
                                    "loop_playlist()" => self.r#loop("playlist"),
                                    _ => {}
                                }
                            }

                            // Spawn a new command process based on exec
                            if let Some(exec) = exec {
                                let parts: Vec<&str> = exec.split_whitespace().collect();
                                if let Some(command) = parts.get(0) {
                                    let _ = Command::new(command)
                                        .args(&parts[1..])
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn();
                                }
                            }
                        }
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

    fn stop(&self) {
        if let Some(player) = &self.player {
            player.stop().expect("Failed to stop player");
        }
    }

    fn play(&self) {
        if let Some(player) = &self.player {
            player.play().expect("Failed to play player");
        }
    }

    fn pause(&self) {
        if let Some(player) = &self.player {
            player.pause().expect("Failed to pause player");
        }
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

    fn shuffle(&self, shuffle: &str) {
        if let Some(player) = &self.player {
            if player.can_shuffle().expect("Failed to get if player can shuffle") {
                match shuffle {
                    "off" => player.set_shuffle(false).expect("Failed to set off shuffle"),
                    "toggle" => player.set_shuffle(!player.get_shuffle().expect("Failed to get shuffle state")).expect("Failed to toggle shuffle"),
                    "on" => player.set_shuffle(true).expect("Failed to set on shuffle"),
                    _ => unreachable!()
                }
            }
        }
    }

    fn r#loop(&self, r#loop: &str) {
        if let Some(player) = &self.player {
            if player.can_loop().expect("Failed to get if player can loop") {
                match r#loop {
                    "none" => player.set_loop_status(LoopStatus::None).expect("Failed to set loop to none"),
                    "track" => player.set_loop_status(LoopStatus::Track).expect("Failed to set loop to track"),
                    "playlist" => player.set_loop_status(LoopStatus::Playlist).expect("Failed to set loop to playlist"),
                    _ => unreachable!()
                }
            }
        }
    }
}
