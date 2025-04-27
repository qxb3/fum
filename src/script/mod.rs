mod config;
mod functions;
mod location;

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use config::FumConfig;
use location::UiLocation;
use notify::Watcher;
use ratatui::{layout::Rect, style::Color};
use rhai::{Engine, Scope, AST};
use taffy::TaffyTree;

use crate::{
    cover::Cover,
    state::FumState,
    track::Track,
    widget::{FumWidget, SliderDataSource},
    FumResult,
};

/// Type alias for the script config state.
pub type ScriptConfig = Arc<Mutex<FumConfig>>;

/// Type alias for TaffyTree wrapped on arc mutex.
pub type ScriptTaffy = Arc<Mutex<TaffyTree<FumWidget>>>;

/// Type alias for the script ui state.
pub type ScriptUi = Arc<Mutex<Vec<(Rect, FumWidget)>>>;

/// Type alias for script persisten variables.
pub type ScriptVars = Arc<Mutex<HashMap<String, rhai::Dynamic>>>;

/// Script event.
pub enum ScriptEvent {
    /// Triggers when the script uses SET_VAR() function to update a persistent variable.
    SetVar,

    /// Triggers when the script has been modified.
    ScriptModified,
}

/// Fum script.
#[allow(dead_code)]
pub struct Script<'a> {
    /// The script config path.
    pub config_path: PathBuf,

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

    /// Script persistent variables.
    pub vars: ScriptVars,

    /// Script event sender.
    script_sender: tokio::sync::mpsc::UnboundedSender<ScriptEvent>,

    /// Script event receiver.
    script_receiver: tokio::sync::mpsc::UnboundedReceiver<ScriptEvent>,

    /// Config watcher sender.
    config_watcher_sender: std::sync::mpsc::Sender<notify::Result<notify::Event>>,

    /// Config watcher receiver.
    config_watcher_receiver: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,

    /// Config watcher.
    config_watcher: notify::INotifyWatcher,
}

impl<'a> Script<'a> {
    /// Creates a new script, loading from file and passing in the state to be used in rhai functions.
    pub fn new(config_path: &PathBuf, state: FumState) -> FumResult<Self> {
        // Rhai engine.
        let mut engine = Engine::new();
        engine.set_max_expr_depths(999, 999); // Have enough expr depths.

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

        // Container justify content variables.
        scope.push("JUSTIFY_START", taffy::JustifyContent::Start);
        scope.push("JUSTIFY_CENTER", taffy::JustifyContent::Center);
        scope.push("JUSTIFY_END", taffy::JustifyContent::End);
        scope.push("JUSTIFY_BETWEEN", taffy::JustifyContent::SpaceBetween);
        scope.push("JUSTIFY_EVENLY", taffy::JustifyContent::SpaceEvenly);
        scope.push("JUSTIFY_AROUND", taffy::JustifyContent::SpaceAround);

        // Slider data source variables.
        scope.push("SOURCE_PROGRESS", SliderDataSource::Progress);
        scope.push("SOURCE_VOLUME", SliderDataSource::Volume);

        // Color variables.
        scope.push("RESET", Color::Reset);
        scope.push("BLACK", Color::Black);
        scope.push("RED", Color::Red);
        scope.push("GREEN", Color::Green);
        scope.push("YELLOW", Color::Yellow);
        scope.push("BLUE", Color::Blue);
        scope.push("MAGENTA", Color::Magenta);
        scope.push("CYAN", Color::Cyan);
        scope.push("GRAY", Color::Gray);
        scope.push("DARK_GRAY", Color::DarkGray);
        scope.push("LIGHT_RED", Color::LightRed);
        scope.push("LIGHT_GREEN", Color::LightGreen);
        scope.push("LIGHT_YELLOW", Color::LightYellow);
        scope.push("LIGHT_BLUE", Color::LightBlue);
        scope.push("LIGHT_MAGENTA", Color::LightMagenta);
        scope.push("LIGHT_CYAN", Color::LightCyan);
        scope.push("WHITE", Color::White);

        // Push the default track metadata into the scope.
        update_scope_track_meta(&mut scope, &Track::new());

        // Push the default avg color into the scope.
        update_cover_avg_color(&mut scope, None);

        // Script config.
        let config = Arc::new(Mutex::new(FumConfig::new()));

        // Taffy layout engine.
        let taffy = Arc::new(Mutex::new(TaffyTree::new()));

        // Script ui.
        let ui = Arc::new(Mutex::new(Vec::new()));

        // Script persistent variables.
        let vars = Arc::new(Mutex::new(HashMap::new()));

        // Sender & Receiver for script event.
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        // Config watcher channels.
        let (config_watcher_sender, config_watcher_receiver) =
            std::sync::mpsc::channel::<notify::Result<notify::Event>>();

        let script_event_sender = sender.clone();
        let config_watcher = notify::RecommendedWatcher::new(
            move |_| {
                script_event_sender
                    .send(ScriptEvent::ScriptModified)
                    .unwrap();
            },
            notify::Config::default(),
        )?;

        // Register FumWidget type.
        engine.register_type_with_name::<FumWidget>("Widget");

        // Register std Duration along with some of it functions & custom one.
        engine
            .register_type_with_name::<Duration>("Duration")
            .register_fn("as_millis", functions::duration_as_millis())
            .register_fn("as_nanos", functions::duration_as_nanos())
            .register_fn("as_secs", functions::duration_as_secs())
            .register_fn("is_zero", functions::duration_is_zero())
            .register_fn("fmt", functions::duration_fmt())
            .register_fn("fmt", functions::duration_fmt_ext());

        // Register CONFIG, UI & Widget functions.
        engine
            .register_fn("CONFIG", functions::config(Arc::clone(&config)))
            .register_fn("UI", functions::ui_opts(Arc::clone(&taffy), Arc::clone(&ui)))
            .register_fn("UI", functions::ui(Arc::clone(&taffy), Arc::clone(&ui)))
            .register_fn("UI", functions::ui_ext_opts(Arc::clone(&taffy), Arc::clone(&ui)))
            .register_fn("Container", functions::container_opts())
            .register_fn("Container", functions::container())
            .register_fn("Container", functions::container_ext_opts())
            .register_fn("ContainerCenter", functions::container_center())
            .register_fn("ContainerCenter", functions::container_center_ext_opts())
            .register_fn("ContainerEnd", functions::container_end())
            .register_fn("ContainerEnd", functions::container_end_ext_opts())
            .register_fn("CoverImage", functions::cover_image())
            .register_fn("Label", functions::label_opts())
            .register_fn("Label", functions::label())
            .register_fn("Label", functions::label_ext_opts())
            .register_fn("LabelVertical", functions::label_vertical())
            .register_fn("LabelVertical", functions::label_vertical_ext_opts())
            .register_fn("Button", functions::button_opts())
            .register_fn("Button", functions::button())
            .register_fn("Button", functions::button_ext_opts())
            .register_fn("ButtonVertical", functions::button_vertical())
            .register_fn("ButtonVertical", functions::button_vertical_ext_opts())
            .register_fn("Slider", functions::slider_opts());

        // Register player control functions.
        engine
            .register_fn("PLAY", functions::play(Arc::clone(&state)))
            .register_fn("PLAY_PAUSE", functions::play_pause(Arc::clone(&state)))
            .register_fn("PAUSE", functions::pause(Arc::clone(&state)))
            .register_fn("PREV", functions::prev(Arc::clone(&state)))
            .register_fn("NEXT", functions::next(Arc::clone(&state)))
            .register_fn("STOP", functions::stop(Arc::clone(&state)));

        // Register vars function.
        engine
            .register_fn("DEF_VAR", functions::define_var(Arc::clone(&vars)))
            .register_fn("SET_VAR", functions::set_var(Arc::clone(&vars), sender.clone()))
            .register_fn("GET_VAR", functions::get_var(Arc::clone(&vars)));

        // Compile the script into ast.
        let ast = engine
            .compile_file(config_path.clone())
            .map_err(|err| format!("Error parsing config script: {err}"))?;

        Ok(Self {
            config_path: config_path.clone(),
            engine,
            scope,
            ast,
            config,
            ui,
            vars,
            script_sender: sender,
            script_receiver: receiver,
            config_watcher_sender,
            config_watcher_receiver,
            config_watcher,
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
        // Updates the POSITION.
        self.scope.set_value("POSITION", position);

        // Re-execute the script.
        self.execute()?;

        Ok(())
    }

    /// Updates only the track REMAINING_LENGTH variable from the script.
    pub fn update_remaining_length(
        &mut self,
        length: Duration,
        position: Duration,
    ) -> FumResult<()> {
        let remaining_length = get_remaining_length(length, position);
        self.scope.set_value("REMAINING_LENGTH", remaining_length);

        Ok(())
    }

    /// Updates only the COVER_AVG_COLOR variable from script.
    pub fn update_cover_avg_color(&mut self, cover: Option<&Cover>) -> FumResult<()> {
        update_cover_avg_color(&mut self.scope, cover);

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

    /// Re-compiles the ast.
    pub fn recompile(&mut self) -> FumResult<()> {
        self.ast = self
            .engine
            .compile_file(self.config_path.clone())
            .map_err(|err| format!("Error parsing config script: {err}"))?;

        Ok(())
    }

    /// Receive the script events..
    pub async fn recv(&mut self) -> FumResult<ScriptEvent> {
        self.script_receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive an event from script",
            )))
    }

    /// Watches the script config for changes.
    pub async fn watch_config(&mut self) -> FumResult<()> {
        self.config_watcher
            .watch(self.config_path.as_path(), notify::RecursiveMode::NonRecursive)?;

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
    let remaining_length = get_remaining_length(length, position);

    scope.set_value("TITLE", title);
    scope.set_value("ALBUM", album);
    scope.set_value("ARTISTS", artists);
    scope.set_value("LENGTH", length);
    scope.set_value("ART_URL", art_url);
    scope.set_value("PLAYBACK_STATUS", playback_status);
    scope.set_value("SHUFFLE", shuffle);
    scope.set_value("VOLUME", volume);
    scope.set_value("POSITION", position);
    scope.set_value("REMAINING_LENGTH", remaining_length);
}

/// A helper function to update the avg color variable from the script.
pub fn update_cover_avg_color(scope: &mut Scope, cover: Option<&Cover>) {
    if let Some(cover) = cover {
        scope.push("COVER_AVG_COLOR", cover.avg_color);
    } else {
        scope.push("COVER_AVG_COLOR", Color::Reset);
    }
}

// Gets the remaining length by subtracting length - position.
// If length isnt greather than position, Default to 0.
pub fn get_remaining_length(length: Duration, position: Duration) -> Duration {
    if length > position {
        length - position
    } else {
        Duration::from_secs(0)
    }
}
