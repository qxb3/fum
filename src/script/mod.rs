mod engine;
mod functions;
mod scope;
mod watcher;

use std::path::PathBuf;

use engine::Engine;
use scope::GlobalVars;
use watcher::ConfigWatcher;

use crate::{
    event::{EventSender, ScriptEvent},
    state::State,
    track::Track,
    FumResult,
};

/// Manages the config script.
pub struct Script<'a> {
    /// The script engine.
    engine: Engine,

    /// Where global variables will be put.
    global_vars: GlobalVars<'a>,

    /// Config watcher.
    config_watcher: ConfigWatcher,

    /// Path to the config script.
    config_path: PathBuf,

    /// The centralize event manager sender.
    event_sender: EventSender,
}

impl<'a> Script<'a> {
    pub fn new(event_sender: EventSender, config_path: PathBuf) -> FumResult<Self> {
        let engine = Engine::new(event_sender.clone(), config_path.clone())?;
        let mut global_vars = GlobalVars::new();
        let config_watcher = ConfigWatcher::new(event_sender.clone())?;

        // Sets the global track variables on default track.
        global_vars.set_track(&Track::default());

        Ok(Self {
            global_vars,
            engine,
            config_watcher,
            config_path,
            event_sender,
        })
    }

    /// Handle the script events.
    pub async fn handle(&mut self, state: &mut State, event: ScriptEvent) -> FumResult<()> {
        match event {
            ScriptEvent::ConfigUpdated(config) => state.set_config(config),
            ScriptEvent::LayoutUpdated(layout) => state.set_layout(layout),
            ScriptEvent::ConfigModified => {
                // Re-compile the script
                if let Err(err) = self.compile() {
                    self.event_sender.send(Err(err))?;
                    return Ok(());
                }

                // Re-execute the script.
                if let Err(err) = self.execute() {
                    self.event_sender.send(Err(err))?;
                    return Ok(());
                }

                // Sets the error to None if both compile & executes run successfuly.
                state.set_error(None);
            }
        }

        Ok(())
    }

    /// Compiles the script in engine.
    pub fn compile(&mut self) -> FumResult<()> {
        self.engine.compile()?;

        Ok(())
    }

    /// Executes the script in engine.
    pub fn execute(&mut self) -> FumResult<()> {
        self.engine.execute(&mut self.global_vars)?;

        Ok(())
    }

    /// Watches the config for changes.
    pub fn watch_config(&mut self) -> FumResult<()> {
        self.config_watcher.watch(&self.config_path)?;

        Ok(())
    }
}
