use ratatui::crossterm::{event::DisableMouseCapture, execute};
use std::io::stdout;

#[macro_export]
macro_rules! send_message {
    ($tx:expr, $msg:expr) => {
        $tx.send($msg).unwrap()
    };
}

#[macro_export]
macro_rules! send_err {
    ($tx:expr, $err:expr) => {
        $tx.send(Message::Err($err)).unwrap()
    };
}

#[macro_export]
macro_rules! send_dbg {
    ($tx:expr, $msg:expr) => {
        $tx.send(Message::Dbg($msg)).unwrap()
    };
}

pub fn truncate(string: &str, width: usize) -> String {
    if string.chars().count() < width {
        string.to_string()
    } else {
        let truncated: String = string.chars().take(width).collect();
        format!("{}...", truncated)
    }
}

pub fn restore() {
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)
        .expect("Failed to disable mouse capture.");
}

pub mod player {
    use std::{io::Cursor, time::Duration};

    use image::{DynamicImage, ImageReader};
    use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder};
    use reqwest::header::RANGE;

    use crate::Meta;

    pub fn get_active_player() -> Result<Player, String> {
        PlayerFinder::new()
            .map_err(|err| format!("Failed to connect to D-Bus: {:?}.", err))?
            .find_active()
            .map_err(|err| format!("Failed to find active player: {:?}.", err))
    }

    pub fn get_metadata(player: &Player) -> Result<Metadata, String> {
        player
            .get_metadata()
            .map_err(|err| format!("Failed to get the player's metadata: {err}."))
    }

    pub fn get_title(metadata: &Metadata) -> Result<String, String> {
        metadata
            .title()
            .map(|t| t.to_string())
            .ok_or("Failed to get xesam:title.".to_string())
    }

    pub fn get_artists(metadata: &Metadata) -> Result<Vec<String>, String> {
        metadata
            .artists()
            .map(|a| a.iter().map(|a| a.to_string()).collect())
            .ok_or("Failed to get xesam:title.".to_string())
    }

    pub fn get_status(player: &Player) -> Result<PlaybackStatus, String> {
        player
            .get_playback_status()
            .map_err(|err| format!("Failed to get player playback_status: {:?}.", err))
    }

    pub fn get_status_icon<'a>(status: &PlaybackStatus) -> &'a str {
        match status {
            PlaybackStatus::Stopped => "󰓛",
            PlaybackStatus::Playing => "󰏤",
            PlaybackStatus::Paused  => "󰐊"
        }
    }

    pub fn get_length(metadata: &Metadata) -> Result<Duration, String> {
        metadata
            .length()
            .ok_or("Failed to get mpris:length.".to_string())
    }

    pub fn get_cover_art(metadata: &Metadata) -> Result<DynamicImage, String> {
        let art_url = metadata.get("mpris:artUrl")
            .ok_or("Failed to get mpris:artUrl")?;

        if let mpris::MetadataValue::String(art_url) = art_url {
            let client = reqwest::blocking::Client::new();
            let resp = client
                .get(art_url)
                .header(RANGE, "bytes=0-1048576")
                .send()
                .map_err(|_| "Failed to fetch art url.".to_string())?;

            let bytes = resp.bytes()
                .map_err(|_| "Failed to get art image bytes.".to_string())?;

            let cover_art = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .map_err(|_| "Unknown image file_type.".to_string())?
                .decode()
                .map_err(|_| "Failed to decode image.".to_string())?;

            return Ok(cover_art)
        }

        Err("mpris:artUrl is not a string.".to_string())
    }

    pub fn get_meta(player: &Player) -> Result<Meta, String> {
        let metadata = get_metadata(player)?;
        let title = get_title(&metadata)?;
        let artists = get_artists(&metadata)?;
        let status = get_status(player)?;
        let length = get_length(&metadata)?;
        let cover_art = get_cover_art(&metadata)?;

        Ok(Meta {
            title,
            artists,
            status,
            length,
            cover_art
        })
    }
}
