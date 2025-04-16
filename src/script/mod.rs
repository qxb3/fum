use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use config::FumConfig;
use location::UiLocation;
use ratatui::layout::Rect;
use rhai::{Engine, Scope, AST};
use taffy::TaffyTree;

use crate::{
    track::Track, utils::duration::format_duration, widget::FumWidget, FumResult,
};

mod config;
mod functions;
mod location;

/// Type alias for the script config state.
pub type ScriptConfig = Arc<Mutex<FumConfig>>;

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

    /// Script config.
    pub config: ScriptConfig,

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

        // UI() function location variables.
        scope.push("UI_TOP", UiLocation::Top);
        scope.push("UI_TOP_LEFT", UiLocation::TopLeft);
        scope.push("UI_TOP_RIGHT", UiLocation::TopRight);
        scope.push("UI_BOTTOM", UiLocation::Bottom);
        scope.push("UI_BOTTOM_LEFT", UiLocation::BottomLeft);
        scope.push("UI_BOTTOM_RIGHT", UiLocation::BottomRight);
        scope.push("UI_LEFT", UiLocation::Left);
        scope.push("UI_RIGHT", UiLocation::Right);
        scope.push("UI_CENTER", UiLocation::Center);

        // Container direction variables.
        scope.push("VERTICAL", taffy::FlexDirection::Column);
        scope.push("HORIZONTAL", taffy::FlexDirection::Row);

        // Container alignment variables.
        scope.push("ALIGN_START", taffy::AlignItems::Start);
        scope.push("ALIGN_CENTER", taffy::AlignItems::Center);
        scope.push("ALIGN_END", taffy::AlignItems::End);

        // Push the default track metadata into the scope.
        update_scope_track_meta(&mut scope, &Track::new());

        // Script config.
        let config = Arc::new(Mutex::new(FumConfig::new()));

        // Taffy layout engine.
        let taffy = Arc::new(Mutex::new(TaffyTree::new()));

        // Script ui.
        let ui = Arc::new(Mutex::new(Vec::new()));

        // Register FumWidget type.
        engine.register_type_with_name::<FumWidget>("Widget");

        // Register std Duration along with some of it functions & custom one.
        engine
            .register_type_with_name::<Duration>("Duration")
            .register_fn("as_millis", |duration: &mut Duration| duration.as_millis())
            .register_fn("as_nanos", |duration: &mut Duration| duration.as_nanos())
            .register_fn("as_secs", |duration: &mut Duration| duration.as_secs())
            .register_fn("is_zero", |duration: &mut Duration| duration.is_zero())
            .register_fn("fmt", |d: &mut Duration| format_duration(d, false))
            .register_fn("fmt_ext", |d: &mut Duration| format_duration(d, true));

        // Register ui functions.
        engine
            .register_fn("UI", functions::fum_ui(taffy.clone(), ui.clone()))
            .register_fn("Container", functions::container()) // Container without opts.
            .register_fn("Container", functions::container_opts()) // Container with opts.
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
            config,
            ui,
        })
    }

    /// Updates the track variables from the script.
    pub fn update_track(&mut self, track: &Track) -> FumResult<()> {
        update_scope_track_meta(&mut self.scope, track);

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

/// A helper function to update the track variables from the script.
fn update_scope_track_meta(scope: &mut Scope, track: &Track) {
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
}
