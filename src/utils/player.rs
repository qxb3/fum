use std::{fs, io::Cursor, str::FromStr, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use image::ImageReader;
use mpris::{Metadata, MetadataValue, PlaybackStatus, Player, PlayerFinder};
use ratatui_image::picker::Picker;
use reqwest::{header::RANGE, Url};

use crate::{config::Config, meta::{CoverArt, Meta}};

pub fn get_player(config: &Config) -> Result<Player, String> {
    let players = PlayerFinder::new()
        .map_err(|err| format!("Failed to connect to D-Bus: {:?}.", err))?
        .find_all()
        .map_err(|err| format!("There is no any active players: {:?}.", err))?;

    for player in players {
        let identity = player.identity().to_lowercase();
        let bus_name = player.bus_name();

        if config.players.iter().any(|p|
            p.to_lowercase() == identity.to_lowercase() ||
            bus_name.starts_with(p)
        ) {
            return Ok(player);
        }
    }

    // Find the most likely player to be used
    if config.use_active_player {
        return PlayerFinder::new()
            .map_err(|err| format!("Failed to connect to D-Bus: {:?}", err))?
            .find_active()
            .map_err(|err| format!("'use-active-player' is set to true but failed to get active player: {err}"));
    }

    Err("Failed to find any specified players.".to_string())
}

pub fn get_metadata(player: &Player) -> Result<Metadata, String> {
    player
        .get_metadata()
        .map_err(|err| format!("Failed to get the player's metadata: {err}"))
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
        .map_err(|err| format!("Failed to get player playback_status: {err}"))
}

pub fn get_status_icon<'a>(status: &PlaybackStatus) -> &'a str {
    match status {
        PlaybackStatus::Stopped => "󰓛",
        PlaybackStatus::Playing => "󰏤",
        PlaybackStatus::Paused  => "󰐊"
    }
}

pub fn get_position(player: &Player) -> Result<Duration, String> {
    player.get_position()
        .map_err(|err| format!("Failed to get player position: {err}"))
}

pub fn get_length(metadata: &Metadata) -> Result<Duration, String> {
    metadata
        .length()
        .ok_or("Failed to get mpris:length".to_string())
}

pub fn get_album(metadata: &Metadata) -> Result<String, String> {
    metadata
        .album_name()
        .map(|a| a.to_string())
        .ok_or("Failed to get xesam:album".to_string())
}

pub fn get_custom_meta(metadata: &Metadata, key: String) -> String {
    let value = metadata.get(&key);

    match value {
        Some(value) => match value {
            MetadataValue::String(str) => str.to_string(),
            MetadataValue::Bool(bool) => bool.to_string(),

            MetadataValue::U8(u8) => u8.to_string(),
            MetadataValue::U16(u16) => u16.to_string(),
            MetadataValue::U32(u32) => u32.to_string(),
            MetadataValue::U64(u64) => u64.to_string(),

            MetadataValue::I16(i16) => i16.to_string(),
            MetadataValue::I32(i32) => i32.to_string(),
            MetadataValue::I64(i64) => i64.to_string(),

            MetadataValue::F64(f64) => f64.to_string(),

            MetadataValue::Unsupported | _ => "!Unsupported".to_string()
        },
        None => "!NotFound".to_string()
    }
}

pub fn get_cover_art(metadata: &Metadata, picker: &Picker, current: Option<&Meta>) -> Result<CoverArt, String> {
    let art_url = metadata
        .get("mpris:artUrl")
        .ok_or("Failed to get mpris:artUrl")?;

    if let mpris::MetadataValue::String(art_url) = art_url {
        if let Some(current) = &current {
            if let Some(current_art) = &current.cover_art {
                if current_art.url == *art_url {
                    return Ok(current_art.clone());
                }
            }
        }

        // Handle file:// scheme
        if art_url.starts_with("file://") {
            let art_path =  Url::from_str(&art_url)
                .map_err(|err| format!("Failed to parse url: {art_url}: {err}"))?
                .to_file_path()
                .map_err(|_| format!("Failed to convert url: {art_url} to file_path"))?;

            let bytes = fs::read(&art_path)
                .map_err(|err| format!("Failed to read art file: {err}"))?;

            let cover_art = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .map_err(|_| "Unknown image file_type".to_string())?
                .decode()
                .map_err(|_| "Failed to decode image".to_string())?;

            return Ok(CoverArt {
                url: art_url.to_string(),
                image: picker.new_resize_protocol(cover_art)
            })
        }

        // Handle base64
        if art_url.starts_with("data:") {
            let base64_data = art_url
                .split_once("base64,")
                .ok_or("Invalid base64 url format")?
            .1;

            let bytes = BASE64_STANDARD.decode(base64_data)
                .map_err(|err| format!("Failed to decode base64 data: {err}"))?;

            let cover_art = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .map_err(|_| "Unknown image file_type".to_string())?
                .decode()
                .map_err(|_| "Failed to decode image".to_string())?;

            return Ok(CoverArt {
                url: art_url.to_string(),
                image: picker.new_resize_protocol(cover_art),
            });
        }

        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(art_url)
            .header(RANGE, "bytes=0-1048576")
            .send()
            .map_err(|_| "Failed to fetch art url".to_string())?;

        let bytes = resp.bytes()
            .map_err(|_| "Failed to get art image bytes".to_string())?;

        let cover_art = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|_| "Unknown image file_type".to_string())?
            .decode()
            .map_err(|_| "Failed to decode image".to_string())?;

        return Ok(CoverArt {
            url: art_url.to_string(),
            image: picker.new_resize_protocol(cover_art)
        })
    }

    Err("mpris:artUrl is not a string.".to_string())
}

pub fn get_meta<'a>(player: &Player, picker: &Picker, current: Option<&Meta>) -> Result<(Meta<'a>, bool), String> {
    let metadata = get_metadata(player)?;
    let title = get_title(&metadata)?;
    let artists = get_artists(&metadata)?;
    let album = get_album(&metadata)?;
    let status = get_status(player)?;
    let status_icon = get_status_icon(&status);
    let position = get_position(player)?;
    let length = get_length(&metadata)?;
    let cover_art = get_cover_art(&metadata, picker, current)?;

    let mut changed = false;

    if let Some(current) = &current {
        if current.title != title ||
        current.artists != artists ||
        current.status != status ||
        current.length != length ||
        position.as_secs() > current.position.as_secs() {
            changed = true;
        }
    }

    Ok((Meta {
        metadata,
        title,
        artists,
        album,
        status,
        status_icon,
        position,
        length,
        cover_art: Some(cover_art)
    }, changed))
}
