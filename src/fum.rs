use core::error;
use std::{io::{stdout, Stdout}, process::{Command, Stdio}, time::Duration};

use crossterm::{event::{self, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind}, execute};
use mpris::Player;
use ratatui::{layout::Position, prelude::CrosstermBackend, Terminal};
use ratatui_image::picker::Picker;

use crate::{action::Action, config::Config, meta::Meta, ui::Ui, utils};

pub type FumResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Fum<'a> {
    config: &'a Config,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub ui: Ui<'a>,
    pub picker: Picker,
    pub player: Option<Player>,
    pub meta: Meta<'a>,
    pub redraw: bool,
    pub exit: bool
}

impl<'a> Fum<'a> {
    pub fn new(config: &'a Config) -> FumResult<Self> {
        let player = Meta::get_player(&config).ok();

        let picker = Picker::from_query_stdio()?;

        let meta = match &player {
            Some(player) => Meta::fetch(player, &picker, None).unwrap_or(Meta::default()),
            None => Meta::default()
        };

        // Enable mouse capture
        execute!(stdout(), EnableMouseCapture)?;

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

    pub fn run(&mut self) -> FumResult<()> {
        while !self.exit {
            if self.redraw {
                self.terminal.draw(|frame| {
                    self.ui.draw(frame, &mut self.meta);
                    self.redraw = false;
                })?;
            }

            self.update_meta();
            self.term_events()?;
        }

        utils::terminal::restore();

        Ok(())
    }

    fn term_events(&mut self) -> FumResult<()> {
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;

            match event {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc |
                        KeyCode::Char('q') => {
                            self.exit = true;
                        },
                        KeyCode::Char('p') => Action::run(Action::Prev, &self.player)?,
                        KeyCode::Char(' ') => Action::run(Action::Pause, &self.player)?,
                        KeyCode::Char('n') => Action::run(Action::Next, &self.player)?,
                        _ => {}
                    }
                },
                Event::Mouse(mouse) if mouse.kind == MouseEventKind::Down(MouseButton::Left) => {
                    for (rect, action, exec) in self.ui.buttons.values() {
                        if rect.contains(Position::new(mouse.column, mouse.row)) {
                            // Execute action
                            if let Some(action) = action {
                                Action::run_str(action.as_str(), &self.player)?;
                            }

                            // Spawn a new command process based on exec
                            if let Some(exec) = exec {
                                let parts: Vec<&str> = exec.split_whitespace().collect();
                                if let Some(command) = parts.get(0) {
                                    let _ = Command::new(command) // Ignore result
                                        .args(&parts[1..])
                                        .stdout(Stdio::null())
                                        .stderr(Stdio::null())
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

        Ok(())
    }

    fn update_meta(&mut self) {
        if let Some(player) = &self.player {
            let meta = Meta::fetch(player, &self.picker, Some(&self.meta))
                .unwrap_or(Meta::default());

            self.redraw = meta.changed;
            self.meta = meta;

            return;
        }

        self.player = Meta::get_player(&self.config).ok();
        self.meta = Meta::default();
    }
}
