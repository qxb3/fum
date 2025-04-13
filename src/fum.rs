use std::{ops::Deref, panic, sync::Arc};

use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{
    cli::CliArgs,
    event::{EventHandler, FumEvent},
    mode::{FumMode, MprisMode, MprisModeEvent},
    script::Script,
    state::State,
    ui, FumResult,
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
        let script = Script::from_file(&args.config_path)?;

        // Creates terminal event handler.
        let event_handler = EventHandler::new(10);

        Ok(Self {
            terminal,
            event_handler,
            state,
            script,
        })
    }

    /// Start Fum.
    pub async fn start(&mut self, mode: FumMode) -> FumResult<()> {
        // Start event handler.
        self.event_handler.handle();

        // Execute the script at start.
        self.script.execute()?;

        let (mpris_mode_tx, mut mpris_mode_rx) = tokio::sync::mpsc::channel(10);

        // Handle the corresponding mode.
        match mode {
            FumMode::Player => {}
            FumMode::Mpris => {
                let mut mpris_mode = MprisMode::new(
                    mpris_mode_tx.clone(),
                    Arc::clone(&self.state.current_player),
                    Arc::clone(&self.state.current_track),
                    Arc::clone(&self.state.current_cover),
                )
                .await?;

                mpris_mode.handle().await?;
            }
        }

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
                Some(mpris_mode_event) = mpris_mode_rx.recv() => {
                    match mpris_mode_event {
                        // Updates the script track variables when the track metadata changes.
                        MprisModeEvent::PlayerTrackMetaChanged => {
                            let current_track_arc = Arc::clone(&self.state.current_track);
                            let current_track = current_track_arc.lock().await;

                            self.script.update_track(current_track.deref())?;
                        }

                        // Updates the script track POSITION variable when the track position changes.
                        MprisModeEvent::PlayerPositionChanged => {
                            let current_track_arc = Arc::clone(&self.state.current_track);
                            let current_track = current_track_arc.lock().await;

                            self.script.update_position(current_track.position.clone())?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle tick event.
    async fn tick(&mut self) -> FumResult<()> {
        let ui_arc = Arc::clone(&self.script.ui);
        let ui = ui_arc
            .lock()
            .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;

        // Draws the ui.
        ui::draw(&mut self.terminal, &mut self.state, ui.deref()).await?;

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
        _mouse: crossterm::event::MouseEvent,
        _button: crossterm::event::MouseButton,
    ) -> FumResult<()> {
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
