use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use ratatui::style::Color;
use rhai::Scope;
use taffy::{AlignItems, JustifyContent};

use crate::track::Track;

/// A wrapper around rhai scope and where global variables will be contained.
pub struct GlobalVars<'a> {
    scope: Scope<'a>,
}

impl<'a> Deref for GlobalVars<'a> {
    type Target = Scope<'a>;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}

impl<'a> DerefMut for GlobalVars<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scope
    }
}

impl<'a> GlobalVars<'a> {
    pub fn new() -> Self {
        let mut scope = Scope::new();

        // // UI() function location variables.
        // scope.push("UI_TOP", UiLocation::Top);
        // scope.push("UI_TOP_LEFT", UiLocation::TopLeft);
        // scope.push("UI_TOP_RIGHT", UiLocation::TopRight);
        // scope.push("UI_BOTTOM", UiLocation::Bottom);
        // scope.push("UI_BOTTOM_LEFT", UiLocation::BottomLeft);
        // scope.push("UI_BOTTOM_RIGHT", UiLocation::BottomRight);
        // scope.push("UI_LEFT", UiLocation::Left);
        // scope.push("UI_RIGHT", UiLocation::Right);
        // scope.push("UI_CENTER", UiLocation::Center);

        // Container alignment variables.
        scope.push("ALIGN_START", AlignItems::Start);
        scope.push("ALIGN_CENTER", AlignItems::Center);
        scope.push("ALIGN_END", AlignItems::End);

        // Container justify content variables.
        scope.push("JUSTIFY_START", JustifyContent::Start);
        scope.push("JUSTIFY_CENTER", JustifyContent::Center);
        scope.push("JUSTIFY_END", JustifyContent::End);
        scope.push("JUSTIFY_BETWEEN", JustifyContent::SpaceBetween);
        scope.push("JUSTIFY_EVENLY", JustifyContent::SpaceEvenly);
        scope.push("JUSTIFY_AROUND", JustifyContent::SpaceAround);

        // // Slider data source variables.
        // scope.push("SOURCE_PROGRESS", SliderDataSource::Progress);
        // scope.push("SOURCE_VOLUME", SliderDataSource::Volume);

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

        Self { scope }
    }

    /// Sets the track global variables.
    pub fn set_track(&mut self, track: &Track) {
        let title = track.title.to_string();
        let album = track.album.to_string();
        let artists = track.artists.join(", ");
        let length = track.length.clone();
        let art_url = track.art_url.clone().unwrap_or("".into());
        let playback_status = track.playback_status.to_string();
        let shuffle = track.shuffle.clone();
        let volume = track.volume.clone();
        let position = track.position.clone();
        let remaining_length =
            if length > position { length - position } else { Duration::from_secs(0) };

        self.scope.set_value("TITLE", title);
        self.scope.set_value("ALBUM", album);
        self.scope.set_value("ARTISTS", artists);
        self.scope.set_value("LENGTH", length);
        self.scope.set_value("ART_URL", art_url);
        self.scope.set_value("PLAYBACK_STATUS", playback_status);
        self.scope.set_value("SHUFFLE", shuffle);
        self.scope.set_value("VOLUME", volume);
        self.scope.set_value("POSITION", position);
        self.scope.set_value("REMAINING_LENGTH", remaining_length);
    }
}
