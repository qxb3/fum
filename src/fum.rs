use std::{panic, sync::Arc};

use ratatui::{layout::Position, prelude::CrosstermBackend, Terminal};

use crate::{
    cli::CliArgs,
    event::{EventHandler, FumEvent},
    mode::{FumMode, FumModeEvent, FumModes, MprisMode, MprisModeEvent, PlayerMode},
    script::Script,
    state::State,
    ui,
    widget::FumWidget,
    FumResult,
};

/// Fum TUI App.
pub struct Fum<'a> {
    /// ratatui terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Event handler.
    event_handler: EventHandler,

    /// Application state.
    state: State,

    /// Config script.
    script: Script<'a>,
}

impl<'a> Fum<'a> {
    /// Creates new Fum TUI.
    pub async fn new(args: &CliArgs) -> FumResult<Self> {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let _ = Fum::restore();

            panic_hook(panic);
        }));

        // Enables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

        // Creates a new state.
        let state = State::new();

        // Initialize ratatui.
        let terminal = ratatui::init();

        // Creates a script from file.
        let mut script = Script::from_file(&args.config_path)?;
        script.execute()?; // Executes the script to populate the config state.

        // Acquire lock for config state.
        let config_arc = Arc::clone(&script.config);
        let config = config_arc
            .lock()
            .map_err(|err| format!("Failed to acquire lock for config state: {err}"))?;

        // Creates Terminal event handler.
        let event_handler = EventHandler::new(config.fps);

        Ok(Self {
            terminal,
            event_handler,
            state,
            script,
        })
    }

    /// Start Fum.
    pub async fn start(&mut self, mode: FumModes) -> FumResult<()> {
        // Start event handler.
        self.event_handler.handle();

        // Execute the script at start.
        self.script.execute()?;

        // Get the according mode.
        let mut mode: Box<dyn FumMode> = match mode {
            FumModes::Player => {
                let player_mode = PlayerMode::new();
                Box::new(player_mode)
            }

            FumModes::Mpris => {
                let current_player = Arc::clone(&self.state.current_player);
                let current_track = Arc::clone(&self.state.current_track);
                let current_cover = Arc::clone(&self.state.current_cover);

                let mpris_mode =
                    MprisMode::new(current_player, current_track, current_cover).await?;

                Box::new(mpris_mode)
            }
        };

        // Start the mode.
        mode.start().await?;

        // Read events and execute while we running.
        while !self.state.exit {
            tokio::select! {
                // Read crossterm terminal events.
                term_event = self.event_handler.next() => {
                    match term_event? {
                        FumEvent::Tick => self.tick().await?,
                        FumEvent::KeyPress(key) => self.keypress(key).await?,
                        FumEvent::MouseClick(mouse, button) => {
                            self.mouse_click(mouse, button).await?
                        },
                        FumEvent::Resize(_, _) => {
                            // Re-execute the script when the terminal resized so the ui positioning updates.
                            self.script.execute()?;
                        },
                    }
                }

                // Read Mpris mode events.
                mpris_mode_event = mode.recv() => {
                    match mpris_mode_event? {
                        // Updates the script track variables when the track metadata changes.
                        FumModeEvent::MprisEvent(MprisModeEvent::PlayerTrackMetaChanged) => {
                            let current_track = self.state.current_track.lock().await;
                            self.script.update_track(&*current_track)?;
                        }

                        // Updates the script track POSITION variable when the track position changes.
                        FumModeEvent::MprisEvent(MprisModeEvent::PlayerPositionChanged) => {
                            let current_track = self.state.current_track.lock().await;
                            self.script.update_position(current_track.position.clone())?;
                        }

                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle tick event.
    async fn tick(&mut self) -> FumResult<()> {
        // Acquire lock for script ui state.
        let ui = self
            .script
            .ui
            .lock()
            .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;

        // Draws the ui.
        ui::draw(&mut self.terminal, &mut self.state, &*ui).await?;

        Ok(())
    }

    /// Handle keypress event.
    async fn keypress(&mut self, key: crossterm::event::KeyEvent) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => self.exit()?,

            _ => {}
        }

        Ok(())
    }

    /// Handle mouse click event.
    async fn mouse_click(
        &mut self,
        mouse: crossterm::event::MouseEvent,
        button: crossterm::event::MouseButton,
    ) -> FumResult<()> {
        if button == crossterm::event::MouseButton::Left {
            // Acquire lock for script ui state.
            let ui = self
                .script
                .ui
                .lock()
                .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;

            // Check for button rects if its been clicked.
            for (rect, widget) in &*ui {
                match widget {
                    FumWidget::Button { func, .. } => {
                        let mouse_pos = Position::new(mouse.column, mouse.row);

                        // If clicked, call the button function.
                        if rect.contains(mouse_pos) {
                            self.script.button_clicked(func)?;
                        }
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Restore terminal state.
    fn restore() -> FumResult<()> {
        // Restore terminal.
        ratatui::restore();

        // Disables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

        Ok(())
    }

    /// Exits out of fum.
    fn exit(&mut self) -> FumResult<()> {
        Fum::restore()?;

        self.state.exit = true;

        Ok(())
    }
}
