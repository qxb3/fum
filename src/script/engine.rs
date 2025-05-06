use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    time::Duration,
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

        // Register custom global functions.
        engine
            // Top level functions.
            .register_fn("CONFIG", functions::config_function(event_sender.clone()))
            .register_fn("LAYOUT", functions::layout_function_raw(event_sender.clone()))
            .register_fn("LAYOUT", functions::layout_function(event_sender.clone()))
            .register_fn("LAYOUT", functions::layout_function_ext(event_sender.clone()))
            // Widget functions.
            .register_fn("Label", functions::label_function_raw(event_sender.clone()))
            .register_fn("Label", functions::label_function(event_sender.clone()))
            .register_fn("Label", functions::label_function_ext(event_sender.clone()))
            .register_fn("Button", functions::button_function_raw(event_sender.clone()))
            .register_fn("Button", functions::button_function(event_sender.clone()))
            // Duration type functions.
            .register_type_with_name::<Duration>("Duration")
            .register_fn("as_millis", functions::duration_as_millis())
            .register_fn("as_nanos", functions::duration_as_nanos())
            .register_fn("as_secs", functions::duration_as_secs())
            .register_fn("is_zero", functions::duration_is_zero())
            .register_fn("fmt", functions::duration_fmt())
            .register_fn("fmt", functions::duration_fmt_ext());

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

    /// Gets the internal rhai engine.
    pub fn rhai_engine(&self) -> &rhai::Engine {
        &self.engine
    }

    /// Gets the AST.
    pub fn ast(&self) -> &rhai::AST {
        &self.ast
    }
}
