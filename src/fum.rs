use std::{panic, sync::Arc, time::Duration};

use crossterm::terminal::{Clear, ClearType};
use ratatui::{layout::Position, prelude::CrosstermBackend, Terminal};

use crate::{
    cli::CliArgs,
    event::{EventHandler, FumEvent},
    mode::{FumMode, FumModeEvent, FumModes, MprisMode, MprisModeEvent, PlayerMode},
    script::{Script, ScriptEvent},
    state::{FumState, State},
    ui,
    utils::interaction::get_interacted_slider,
    widget::{FumWidget, SliderDataSource},
    FumResult,
};

/// Fum TUI App.
pub struct Fum<'a> {
    /// ratatui terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Event handler.
    event_handler: EventHandler,

    /// Application state.
    state: FumState,

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

        // Creates a new shared state.
        let state = State::new_shared();

        // Initialize ratatui.
        let terminal = ratatui::init();

        // Clear the termianal after ratatui::init()
        // Helps clear the errors when the previous run exits with errors.
        crossterm::execute!(std::io::stdout(), Clear(ClearType::All))?;

        // Creates a script.
        let mut script = Script::new(&args.config_path, Arc::clone(&state))?;
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

        // Watch the config script for changes.
        self.script.watch_config().await?;

        // Get the according mode.
        let mut mode: Box<dyn FumMode + 'static> = match mode {
            FumModes::Player => {
                let player_mode = PlayerMode::new();
                Box::new(player_mode)
            }

            FumModes::Mpris => {
                let mpris_mode = MprisMode::new(Arc::clone(&self.state)).await?;
                Box::new(mpris_mode)
            }
        };

        // Start the mode.
        mode.start().await?;

        while !self.get_should_exit().await {
            tokio::select! {
                // Read crossterm terminal events.
                term_event = self.event_handler.recv() => {
                    match term_event? {
                        FumEvent::Tick => self.tick().await?,
                        FumEvent::KeyPress(key) => self.keypress(key).await?,
                        FumEvent::MouseClick(mou, btn) => self.mouse_click(mou, btn).await?,
                        FumEvent::MouseDrag(mou, btn) => self.mouse_drag(mou, btn).await?,
                        FumEvent::MouseUp(btn) => self.mouse_up(btn).await?,
                        FumEvent::Resize(_, _) => {
                            // Re-execute the script when the terminal resized so the ui positioning updates.
                            self.script.execute()?;
                        },
                    }
                }

                // Read script events.
                script_event = self.script.recv() => {
                    match script_event? {
                        ScriptEvent::SetVar => {
                            // Re-executes the script for the ui to reflect the change
                            // When a persistent variable has been updated.
                            self.script.execute()?;
                        }

                        ScriptEvent::ScriptModified => {
                            // Re-compiles & Re-execute the script when the script has been modified.
                            self.script.recompile()?;
                            self.script.execute()?;
                        }
                    }
                }

                // Read Mpris mode events.
                mpris_mode_event = mode.recv() => {
                    match mpris_mode_event? {
                        // Updates the script track variables when the track metadata changes.
                        FumModeEvent::MprisEvent(MprisModeEvent::PlayerTrackMetaChanged) => {
                            let state = self.state.lock().await;
                            let current_track = state.get_track();
                            self.script.update_track(current_track)?;
                        }

                        // Updates the script track POSITION & REMAINING_LENGTH variable when the track position changes.
                        FumModeEvent::MprisEvent(MprisModeEvent::PlayerPositionChanged) => {
                            let state = self.state.lock().await;
                            let current_track = state.get_track();
                            self.script.update_position(current_track.position)?;
                            self.script.update_remaining_length(current_track.length, current_track.position)?;
                        }

                        // Updates the script COVER_AVG_COLOR variable when the cover changed.
                        FumModeEvent::MprisEvent(MprisModeEvent::CoverChanged) => {
                            let state = self.state.lock().await;
                            let current_cover = state.get_cover();
                            self.script.update_cover_avg_color(current_cover)?;
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
        ui::draw(&mut self.terminal, Arc::clone(&self.state), &*ui).await?;

        Ok(())
    }

    /// Handle keypress event.
    async fn keypress(&mut self, key: crossterm::event::KeyEvent) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => self.exit().await?,

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
        // Only handle the mouse left click.
        if button == crossterm::event::MouseButton::Left {
            // Button clicked interaction.
            self.button_clicked_interaction(mouse).await?;
        }

        Ok(())
    }

    /// Handle mouse drag event.
    async fn mouse_drag(
        &mut self,
        mouse: crossterm::event::MouseEvent,
        button: crossterm::event::MouseButton,
    ) -> FumResult<()> {
        // Only handle the mouse left drag.
        if button == crossterm::event::MouseButton::Left {
            // Slider drag interaction.
            self.slider_drag_interaction(mouse).await?;
        }

        Ok(())
    }

    /// Handle mouse up event.
    async fn mouse_up(&mut self, button: crossterm::event::MouseButton) -> FumResult<()> {
        // Only handle the mouse left up.
        if button == crossterm::event::MouseButton::Left {
            // End drag states on mouse up event.
            let mut state = self.state.lock().await;
            state.end_drag();
        }

        Ok(())
    }

    pub async fn button_clicked_interaction(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> FumResult<()> {
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

                    // Calls the function when a button has been clicked.
                    if rect.contains(mouse_pos) {
                        func.call(&self.script.engine, &self.script.ast, ())
                            .map_err(|err| format!("Failed to call button function: {err}"))?;
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }

    /// Do the slider drag interaction.
    pub async fn slider_drag_interaction(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> FumResult<()> {
        let mut state = self.state.lock().await;

        // Start drag if not currently.
        if !state.is_dragging() {
            state.start_drag(Position::new(mouse.column, mouse.row));
        }

        // Do stuff if dragging.
        if state.is_dragging() {
            // Update the current drag position.
            state.update_current_drag(Position::new(mouse.column, mouse.row));

            // Gets the drag state.
            let drag_state = state.get_drag();

            // Checks if there is both start & current drags.
            if let Some(start_drag) = drag_state.start_drag {
                if let Some(current_drag) = drag_state.current_drag {
                    // Get the interacted slider based on the ui state.
                    if let Some((slider_rect, slider_source)) =
                        get_interacted_slider(Arc::clone(&self.script.ui), &start_drag)?
                    {
                        // Map the slider's slide into 0 - 1. 0 means the very start, 1 means the very end of the slider.
                        let slider_value = ((current_drag.x as f64 - slider_rect.x as f64)
                            / slider_rect.width as f64)
                            .clamp(0.0, 1.0);

                        match slider_source {
                            // Handle progress slider interaction.
                            SliderDataSource::Progress => {
                                // Gets the track_id, new position & track length.
                                let (track_id, position, length) = {
                                    let current_track = state.get_track();
                                    let length = current_track.length.as_secs(); // Total Length of track in secs.
                                    let position = slider_value * length as f64; // Mul the value above to get the real duration in secs.

                                    (current_track.track_id.to_owned(), position, length)
                                };

                                // This is just to reset / end the drag state's so when you slide the slider
                                // to the end its not gonna skip track a thousand times (you have to drag slide again).
                                if position >= length as f64 {
                                    state.end_drag();
                                }

                                // Update the position if there is a player.
                                let current_player = state.get_player_mut();
                                if let Some(player) = current_player {
                                    if let Some(track_id) = &track_id {
                                        // Set the position.
                                        player
                                            .set_position(
                                                track_id,
                                                Duration::from_secs(position as u64),
                                            )
                                            .await?;

                                        // Re-exceute the script to get update more fastly ig.
                                        self.script.execute()?;
                                    }
                                }
                            }

                            // Handle volume slider interaction.
                            SliderDataSource::Volume => {
                                let current_player = state.get_player_mut();

                                // Updates the volume.
                                if let Some(player) = current_player {
                                    player.set_volume(slider_value).await?;

                                    // Re-exceute the script to get update more fastly ig.
                                    self.script.execute()?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Acquire lock for state and checks if we should exit.
    async fn get_should_exit(&self) -> bool {
        let state = self.state.lock().await;
        state.should_exit()
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
    async fn exit(&mut self) -> FumResult<()> {
        Fum::restore()?;

        // Tell the state to exit.
        let mut state = self.state.lock().await;
        state.exit();

        Ok(())
    }
}
