use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use anyhow::anyhow;
use rhai::AST;

use crate::{event::EventSender, FumResult};

use super::{functions, scope::GlobalVars};

/// A wrapper around rhai engine.
pub struct Engine {
    /// Main rhai engine.
    engine: rhai::Engine,

    /// Config script's compiled ast.
    ast: AST,

    /// Path to the config script.
    config_path: PathBuf,
}

impl Deref for Engine {
    type Target = rhai::Engine;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}

impl Engine {
    pub fn new(event_sender: EventSender, config_path: PathBuf) -> FumResult<Self> {
        let mut engine = rhai::Engine::new();

        // Sets enough expr depths on engine so it won't panic at runtime
        // about having super nested layouts.
        engine.set_max_expr_depths(69420, 69420);

        engine.register_fn("CONFIG", functions::config_function(event_sender.clone()));

        // Compile config script into ast.
        let ast = engine
            .compile_file(config_path.clone())
            .map_err(|err| anyhow!("Failed to compile config script into ast: {err}"))?;

        Ok(Self {
            engine,
            ast,
            config_path,
        })
    }

    /// Compiles the config script.
    pub fn compile(&mut self) -> FumResult<()> {
        self.ast = self
            .engine
            .compile_file(self.config_path.to_path_buf())
            .map_err(|err| anyhow!("Failed to compile config script into ast: {err}"))?;

        Ok(())
    }

    /// Executes the script.
    pub fn execute(&mut self, global_vars: &mut GlobalVars) -> FumResult<()> {
        self.engine
            .run_ast_with_scope(global_vars, &self.ast)
            .map_err(|err| anyhow!("Failed to execute config script: {err}"))?;

        Ok(())
    }
}
