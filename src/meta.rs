use std::{fs, io::{self, Cursor}, str::FromStr, time::Duration};

use base64::{prelude::BASE64_STANDARD, Engine};
use image::ImageReader;
use mpris::{Metadata, MetadataValue, PlaybackStatus, Player, PlayerFinder, TrackID};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use reqwest::{header::RANGE, Url};

use crate::{config::Config, fum::FumResult};

#[derive(Clone)]
pub struct CoverArt {
    pub url: String,
    pub image: StatefulProtocol
}

#[derive(Clone)]
pub struct Meta {
    pub metadata: Metadata,
    pub track_id: Option<TrackID>,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub status: PlaybackStatus,
    pub status_icon: char,
    pub position: Duration,
    pub length: Duration,
    pub volume: f64,
    pub cover_art: Option<CoverArt>,
    pub changed: bool
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            track_id: None,
            title: "No Music".to_string(),
            artists: vec!["Artist".to_string()],
            album: "Album".to_string(),
            status: PlaybackStatus::Stopped,
            status_icon: Meta::get_status_icon(&PlaybackStatus::Stopped),
            position: Duration::from_secs(0),
            length: Duration::from_secs(0),
            volume: 0.0,
            cover_art: None,
            changed: false
        }
    }
}

impl Meta {
    pub fn fetch(player: &Player, picker: &Picker, current: Option<&Self>) -> FumResult<Self> {
        let metadata = Meta::get_metadata(player)?;
        let track_id = Meta::get_trackid(&metadata).ok();
        let title = Meta::get_title(&metadata)?;
        let artists = Meta::get_artists(&metadata).unwrap_or(vec!["Artist".to_string()]);
        let album = Meta::get_album(&metadata).unwrap_or("Album".to_string());
        let status = Meta::get_status(player)?;
        let status_icon = Meta::get_status_icon(&status);
        let position = Meta::get_position(player)?;
        let length = Meta::get_length(&metadata)?;
        let volume = Meta::get_voume(player)?;
        let cover_art = Meta::get_cover_art(&metadata, picker, current).ok();

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

        Ok(Self {
            metadata,
            track_id,
            title,
            artists,
            album,
            status,
            status_icon,
            position,
            length,
            volume,
            cover_art,
            changed
        })
    }

    pub fn get_player(config: &Config) -> FumResult<Player> {
        let finder = PlayerFinder::new()
            .map_err(|err| format!("Failed to connect to D-Bus: {:?}.", err))?;

        let players = finder
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
            let active = finder.find_active()
                .map_err(|err| format!("'use-active-player' is set to true but failed to get active player: {err}"))?;

            return Ok(active);
        }

        Err(Box::new(
            io::Error::new(
                io::ErrorKind::Other,
                "Failed to find any specified players"
            )
        ))
    }

    pub fn get_metadata(player: &Player) -> FumResult<Metadata> {
        let metadata = player.get_metadata()?;
        Ok(metadata)
    }

    pub fn get_trackid(metadata: &Metadata) -> FumResult<TrackID> {
        let trackid = metadata.track_id()
            .ok_or("Failed to get track_id")?;

        Ok(trackid)
    }

    pub fn get_title(metadata: &Metadata) -> FumResult<String> {
        let title = metadata
            .title()
            .map(|t| t.to_string())
            .ok_or("Failed to get xesam:title")?;

        Ok(title)
    }

    pub fn get_artists(metadata: &Metadata) -> FumResult<Vec<String>> {
        let metadata = metadata
            .artists()
            .map(|a| a.iter().map(|a| a.to_string()).collect())
            .ok_or("Failed to get xesam:title.".to_string())?;

        Ok(metadata)
    }

    pub fn get_status(player: &Player) -> FumResult<PlaybackStatus> {
        let status = player
            .get_playback_status()
            .map_err(|err| format!("Failed to get player playback_status: {err}"))?;

        Ok(status)
    }

    pub fn get_status_icon(status: &PlaybackStatus) -> char {
        match status {
            PlaybackStatus::Stopped => '󰓛',
            PlaybackStatus::Playing => '󰏤',
            PlaybackStatus::Paused  => '󰐊'
        }
    }

    pub fn get_position(player: &Player) -> FumResult<Duration> {
        let position = player.get_position()
            .map_err(|err| format!("Failed to get player position: {err}"))?;

        Ok(position)
    }

    pub fn get_length(metadata: &Metadata) -> FumResult<Duration> {
        let length = metadata
            .length()
            .ok_or("Failed to get mpris:length".to_string())?;

        Ok(length)
    }

    pub fn get_album(metadata: &Metadata) -> FumResult<String> {
        let album = metadata
            .album_name()
            .map(|a| a.to_string())
            .ok_or("Failed to get xesam:album".to_string())?;

        Ok(album)
    }

    pub fn get_voume(player: &Player) -> FumResult<f64> {
        let volume = player.get_volume()
            .map_err(|err| format!("Failed to get player volume: {err}"))?;

        Ok(volume)
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

    pub fn get_cover_art(metadata: &Metadata, picker: &Picker, current: Option<&Meta>) -> FumResult<CoverArt> {
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

        Err(Box::new(
            io::Error::new(
                io::ErrorKind::Other,
                "mpris:artUrl is not a string."
            )
        ))
    }
}
