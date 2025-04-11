use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use ratatui::layout::Rect;
use rhai::{Engine, Scope, AST};
use taffy::{NodeId, TaffyTree};

use crate::{
    track::Track, utils::duration::format_duration, widget::FumWidget, FumResult,
};

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
    pub fn from_file<P: Into<PathBuf>>(config_path: P, track: &Track) -> FumResult<Self> {
        // Rhai engine.
        let mut engine = Engine::new();

        // Script scope.
        let mut scope = Scope::new();

        // Push stuff into the scope.
        scope.push("VERTICAL", taffy::FlexDirection::Column);
        scope.push("HORIZONTAL", taffy::FlexDirection::Row);

        // Push track metadata into the scope.
        let title = track.title.to_string();
        let album = track.album.to_string();
        let artists = track.artists.join(", ");
        let length = track.length.clone();
        let art_url = track.art_url.clone().unwrap_or("".into());
        let playback_status = track.playback_status.to_string();
        let shuffle = track.shuffle.clone();
        let volume = track.volume.clone();
        let position = track.position.clone();

        scope.set_value("TITLE", title);
        scope.set_value("ALBUM", album);
        scope.set_value("ARTISTS", artists);
        scope.set_value("LENGTH", length);
        scope.set_value("ART_URL", art_url);
        scope.set_value("PLAYBACK_STATUS", playback_status);
        scope.set_value("SHUFFLE", shuffle);
        scope.set_value("VOLUME", volume);
        scope.set_value("POSITION", position);

        // Taffy layout engine.
        let taffy = Arc::new(Mutex::new(TaffyTree::new()));

        // Script ui.
        let ui = Arc::new(Mutex::new(Vec::new()));

        // Register stuff into the engine.
        engine
            // Register FumWidget type.
            .register_type_with_name::<FumWidget>("Widget")
            // Register std Duration along with some of it functions & custom one.
            .register_type_with_name::<Duration>("Duration")
            .register_fn("as_millis", |duration: &mut Duration| duration.as_millis())
            .register_fn("as_nanos", |duration: &mut Duration| duration.as_nanos())
            .register_fn("as_secs", |duration: &mut Duration| duration.as_secs())
            .register_fn("is_zero", |duration: &mut Duration| duration.is_zero())
            .register_fn("fmt", |duration: &mut Duration| {
                format_duration(duration, false)
            })
            .register_fn("fmt_ext", |duration: &mut Duration| {
                format_duration(duration, true)
            })
            // Register ui functions.
            .register_fn("FUM_UI", functions::fum_ui(taffy.clone(), ui.clone()))
            .register_fn("Container", functions::container())
            .register_fn("CoverImage", functions::cover_image())
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

    /// Updates the track variables from the script.
    pub fn update_track(&mut self, track: &Track) -> FumResult<()> {
        let title = track.title.to_string();
        let album = track.album.to_string();
        let artists = track.artists.join(", ");
        let length = track.length.clone();
        let art_url = track.art_url.clone().unwrap_or("".into());
        let playback_status = track.playback_status.to_string();
        let shuffle = track.shuffle.clone();
        let volume = track.volume.clone();
        let position = track.position.clone();

        self.scope.set_value("TITLE", title);
        self.scope.set_value("ALBUM", album);
        self.scope.set_value("ARTISTS", artists);
        self.scope.set_value("LENGTH", length);
        self.scope.set_value("ART_URL", art_url);
        self.scope.set_value("PLAYBACK_STATUS", playback_status);
        self.scope.set_value("SHUFFLE", shuffle);
        self.scope.set_value("VOLUME", volume);
        self.scope.set_value("POSITION", position);

        // Re-execute the script.
        self.execute()?;

        Ok(())
    }

    /// Updates only the track POSITION variable from the script.
    pub fn update_position(&mut self, position: Duration) -> FumResult<()> {
        self.scope.set_value("POSITION", position);

        // Re-execute the script.
        self.execute()?;

        Ok(())
    }

    // Executes the script.
    pub fn execute(&mut self) -> FumResult<()> {
        self.engine
            .run_ast_with_scope(&mut self.scope, &self.ast)
            .map_err(|err| format!("Error executing config script: {err}"))?;

        Ok(())
    }
}
