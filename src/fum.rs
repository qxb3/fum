use core::error;
use std::{io::{stdout, Stdout}, process::{Command, Stdio}, time::Duration};

use crossterm::{event::{self, EnableMouseCapture, Event, KeyEventKind, MouseButton, MouseEventKind}, execute};
use mpris::Player;
use ratatui::{layout::Position, prelude::CrosstermBackend, Terminal};
use ratatui_image::picker::Picker;

use crate::{action::{Action, VolumeType}, config::{Config, Keybind}, meta::Meta, state::FumState, ui::Ui, utils, widget::{Direction, SliderSource}};

pub type FumResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Fum<'a> {
    config: &'a Config,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub ui: Ui<'a>,
    pub picker: Picker,
    pub player: Option<Player>,
    pub state: FumState,

    // drag state
    pub dragging: bool,
    pub start_drag: Option<Position>,
    pub current_drag: Option<Position>,
    pub drag_action: Option<Action>,

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
            state: FumState::new(meta),

            // drag state
            dragging: false,
            start_drag: None,
            current_drag: None,
            drag_action: None,

            redraw: true, // Draw at startup
            exit: false
        })
    }

    pub fn run(&mut self) -> FumResult<()> {
        while !self.exit {
            if self.redraw {
                self.terminal.draw(|frame| {
                    self.ui.draw(frame, &mut self.state);
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
        if event::poll(Duration::from_millis(self.config.fps))? {
            let event = event::read()?;

            match event {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    for (keybind, action) in self.config.keybinds.iter() {
                        match keybind {
                            Keybind::Many(keybinds) => {
                                for keybind in keybinds {
                                    if key.code == keybind.into_keycode() {
                                        Action::run(action, self)?;
                                    }
                                }
                            },
                            keybind => {
                                if key.code == keybind.into_keycode() {
                                    Action::run(action, self)?;
                                }
                            }
                        }
                    }
                },
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        // Button click mouse left.
                        MouseEventKind::Down(MouseButton::Left) => {
                            if let Some((action, _, exec)) = self.ui.click(mouse.column, mouse.row, &self.state.buttons) {
                                let action = action.to_owned();
                                let exec = exec.to_owned();

                                if let Some(action) = action {
                                    Action::run(&action, self)?;
                                }

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
                        },

                        // Button click mouse right.
                        MouseEventKind::Down(MouseButton::Right) => {
                            if let Some((_, action_secondary, _)) = self.ui.click(mouse.column, mouse.row, &self.state.buttons) {
                                let action_secondary = action_secondary.to_owned();

                                if let Some(action_secondary) = action_secondary {
                                    Action::run(&action_secondary, self)?;
                                }
                            }
                        },

                        // Mouse left drag.
                        MouseEventKind::Drag(MouseButton::Left) => {
                            if !self.dragging && self.start_drag.is_none() {
                                self.dragging = true;
                                self.start_drag = Some(Position::new(mouse.column, mouse.row));
                            }

                            if self.dragging {
                                self.current_drag = Some(Position::new(mouse.column, mouse.row));

                                if let Some(start_drag) = &self.start_drag {
                                    if let Some(current_drag) = &self.current_drag {
                                        if let Some((rect, direction, widget)) = self.ui.drag(start_drag, &self.state.sliders) {
                                            let value: f64 = match direction {
                                                Direction::Horizontal => ((current_drag.x as f64 - rect.x as f64) / rect.width as f64).clamp(0.0, 1.0),
                                                Direction::Vertical => (1.0 - ((current_drag.y as f64 - rect.y as f64) / rect.height as f64)).clamp(0.0, 1.0)
                                            };

                                            match widget {
                                                SliderSource::Progress => {
                                                    let position = value * self.state.meta.length.as_secs() as f64;
                                                    if position >= self.state.meta.length.as_secs() as f64 {
                                                        self.dragging = false;
                                                        self.start_drag = None;
                                                        self.current_drag = None;
                                                    }

                                                    Action::run(&Action::Position(position as u64), self)?;
                                                },
                                                SliderSource::Volume => {
                                                    let volume = value * 100.0;
                                                    Action::run(&Action::Volume(VolumeType::Set(volume)), self)?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },

                        // Mouse left release.
                        MouseEventKind::Up(MouseButton::Left) => {
                            self.dragging = false;
                            self.start_drag = None;
                            self.current_drag = None;
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

        Ok(())
    }

    fn update_meta(&mut self) {
        if let Some(player) = &self.player {
            let meta = Meta::fetch(player, &self.picker, Some(&self.state.meta))
                .unwrap_or(Meta::default());

            self.redraw = meta.changed;
            self.state.meta = meta;

            return;
        }

        self.player = Meta::get_player(&self.config).ok();
        self.state.meta = Meta::default();
    }
}
