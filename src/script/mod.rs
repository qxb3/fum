mod watcher;

use std::path::PathBuf;

use notify::Watcher;
use rhai::{Engine, Scope};
use watcher::ConfigWatcher;

use crate::{
    event::{Event, EventSender, ScriptEvent},
    state::State,
    FumResult,
};

/// Manages the config script.
pub struct Script<'a> {
    /// Where global variables will be put.
    scope: Scope<'a>,

    /// Main rhai engine.
    engine: Engine,

    /// Config watcher.
    config_watcher: ConfigWatcher,
}

impl<'a> Script<'a> {
    pub fn new(event_sender: EventSender) -> FumResult<Self> {
        let scope = Scope::new();

        let mut engine = Engine::new();

        // Have enough expr depths so it won't panic at runtime
        // about having super nested layouts.
        engine.set_max_expr_depths(69420, 69420);

        let config_watcher = ConfigWatcher::new(event_sender.clone())?;

        Ok(Self {
            scope,
            engine,
            config_watcher,
        })
    }

    /// Handle the script events.
    pub async fn handle(&mut self, state: &mut State, event: ScriptEvent) -> FumResult<()> {
        match event {
            ScriptEvent::ConfigUpdated => {}
            ScriptEvent::LayoutUpdated => {}
            ScriptEvent::ConfigModified => {
                // Recompiles & Reexecute the script when the config script has been modified.
                self.recompile()?;
                self.execute()?;
            }
        }

        Ok(())
    }

    /// Executes the script.
    pub fn execute(&mut self) -> FumResult<()> {
        println!("EXECUTE!");

        Ok(())
    }

    /// Recompiles the ast.
    pub fn recompile(&mut self) -> FumResult<()> {
        println!("RECOMPILE!");

        Ok(())
    }

    /// Watches the config for changes.
    pub fn watch_config(&mut self, config_path: &PathBuf) -> FumResult<()> {
        self.config_watcher.watch(config_path)?;

        Ok(())
    }
}
