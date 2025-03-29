use std::path::PathBuf;

use rhai::{Engine, AST};

use crate::{widget::Widget, FumResult};

mod functions;

/// Errors that can happen on the script.
#[derive(Debug, Clone)]
pub enum ScriptErr {
    /// When a type is not what is expected to be.
    InvalidType(String),
}

/// Events that can happen on the script.
#[derive(Debug, Clone)]
pub enum ScriptEvent {
    /// When the script calles the FUM_UI() function.
    UiUpdate(Vec<Widget>),
}

/// Type alias for script event result.
pub type ScriptEventResult = Result<ScriptEvent, ScriptErr>;

/// Type alias for script event channel sender.
pub type ScriptEventSender = tokio::sync::mpsc::UnboundedSender<ScriptEventResult>;

/// Type alias for script event channel receiver.
pub type ScriptEventReceiver = tokio::sync::mpsc::UnboundedReceiver<ScriptEventResult>;

/// Fum script.
pub struct Script {
    /// Event channel sender.
    sender: ScriptEventSender,

    /// Event channel receiver,
    receiver: ScriptEventReceiver,

    /// Rhai engine.
    pub engine: Engine,

    /// Script ast.
    pub ast: AST,
}

impl Script {
    /// Creates a new script, loading from file.
    pub fn new<P: Into<PathBuf>>(config_path: P) -> FumResult<Self> {
        // Creates a new unbounded event channel.
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        // rhai engine.
        let mut engine = Engine::new();

        // Register stuff into the engine.
        engine
            .register_type_with_name::<Widget>("Widget")
            .register_fn("FUM_UI", functions::fum_ui(sender.clone()))
            .register_fn("Container", functions::container(sender.clone()))
            .register_fn("Label", functions::label());

        // Compile the script into ast.
        let ast = engine
            .compile_file(config_path.into())
            .map_err(|err| format!("Error parsing config script: {err}"))?;

        // Run the script and put all the stuff in the scope.
        engine
            .run_ast(&ast)
            .map_err(|err| format!("Error executing config script: {err}"))?;

        Ok(Self {
            sender,
            receiver,
            engine,
            ast,
        })
    }

    /// Receive script events.
    pub async fn next(&mut self) -> FumResult<ScriptEventResult> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive an event",
            )))
    }
}
