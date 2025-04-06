use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use ratatui::layout::Rect;
use rhai::{Engine, Scope, AST};
use taffy::TaffyTree;

use crate::{widget::FumWidget, FumResult};

mod functions;

/// Type alias for TaffyTree wrapped on arc mutex.
pub type ScriptTaffy = Arc<Mutex<TaffyTree<FumWidget>>>;

/// Type alias for the script ui state.
pub type ScriptUi = Arc<Mutex<Vec<(Rect, FumWidget)>>>;

/// Fum script.
pub struct Script<'a> {
    /// Rhai engine.
    pub engine: Engine,

    /// Script scope where global variables be put.
    pub scope: Scope<'a>,

    /// Script ast.
    pub ast: AST,

    /// Script ui.
    pub ui: ScriptUi,
}

impl<'a> Script<'a> {
    /// Creates a new script, loading from file.
    pub fn from_file<P: Into<PathBuf>>(config_path: P) -> FumResult<Self> {
        // Rhai engine.
        let mut engine = Engine::new();

        // Script scope.
        let mut scope = Scope::new();

        // Push stuff into the scope.
        scope.push("VERTICAL", taffy::FlexDirection::Column);
        scope.push("HORIZONTAL", taffy::FlexDirection::Row);

        // Taffy layout engine.
        let taffy = Arc::new(Mutex::new(TaffyTree::new()));

        // Script ui.
        let ui = Arc::new(Mutex::new(Vec::new()));

        // Register stuff into the engine.
        engine
            .register_type_with_name::<FumWidget>("Widget")
            .register_fn("FUM_UI", functions::fum_ui(taffy.clone(), ui.clone()))
            .register_fn("Container", functions::container())
            .register_fn("Label", functions::label());

        // Compile the script into ast.
        let ast = engine
            .compile_file(config_path.into())
            .map_err(|err| format!("Error parsing config script: {err}"))?;

        Ok(Self {
            engine,
            scope,
            ast,
            ui,
        })
    }

    // Executes the script.
    pub fn execute(&mut self) -> FumResult<()> {
        self.engine
            .run_ast_with_scope(&mut self.scope, &self.ast)
            .map_err(|err| format!("Error executing config script: {err}"))?;

        Ok(())
    }
}
