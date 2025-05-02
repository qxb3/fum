#![allow(dead_code)]

use std::{collections::HashMap, time::Duration};

use anyhow::{anyhow, Ok};
use zbus::zvariant;

use crate::FumResult;

/// A custom struct for TrackId.
#[derive(Debug, Clone)]
pub struct TrackId(String);

impl TrackId {
    /// Gets the actual string value of track id.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Represents the metadata of an MPRIS media player.
///
/// This struct stores key-value pairs of metadata properties retrieved from an MPRIS-compatible player.
/// Metadata includes information such as track title, artist, album, and playback details.
#[derive(Debug)]
pub struct PlayerMetadata<'a> {
    metadata: HashMap<String, zvariant::Value<'a>>,
}

impl<'a> PlayerMetadata<'a> {
    /// Creates a new Metadata.
    pub fn new(metadata: HashMap<String, zvariant::Value<'a>>) -> FumResult<Self> {
        Ok(Self { metadata })
    }

    /// Metadata mpris:trackid.
    ///
    /// Returns Err when mpris:trackid is somehow a different type.
    /// Returns None when mpris:trackid doesn't exists.
    pub fn track_id(&self) -> FumResult<Option<TrackId>> {
        self.metadata
            .get("mpris:trackid")
            .map(|track_id| match track_id {
                zvariant::Value::Str(track_id) => Ok(Some(TrackId(track_id.to_string()))),
                zvariant::Value::ObjectPath(track_id) => Ok(Some(TrackId(track_id.to_string()))),
                _ => Err(anyhow!("mpris:trackid is not a object path / string.")),
            })
            .unwrap_or(Ok(None))
    }

    /// Metadata xesam:title.
    ///
    /// Returns Err when xesam:title is somehow a different type.
    /// Returns None when xesam:title doesn't exists.
    pub fn title(&self) -> FumResult<Option<String>> {
        self.metadata
            .get("xesam:title")
            .map(|title| match title {
                zvariant::Value::Str(title) => Ok(Some(title.to_string())),
                _ => Err(anyhow!("xesam:title is not a string.")),
            })
            .unwrap_or(Ok(None))
    }

    /// Metadata xesam:album.
    ///
    /// Returns Err when xesam:album is somehow a different type.
    /// Returns None when xesam:album doesn't exists.
    pub fn album(&self) -> FumResult<Option<String>> {
        self.metadata
            .get("xesam:album")
            .map(|album| match album {
                zvariant::Value::Str(album) => Ok(Some(album.to_string())),
                _ => Err(anyhow!("xesam:album is not a string.")),
            })
            .unwrap_or(Ok(None))
    }

    /// Metadata xesam:artist.
    ///
    /// Returns Err when xesam:artist is somehow a different type.
    /// Returns None when xesam:artist doesn't exists.
    pub fn artists(&self) -> FumResult<Option<Vec<String>>> {
        self.metadata
            .get("xesam:artist")
            .map(|artists| match artists {
                zvariant::Value::Array(artists) => {
                    let artists: Vec<String> = artists
                        .iter()
                        .filter_map(|a| a.downcast_ref::<&str>().map(|s| s.to_string()).ok())
                        .collect();

                    Ok(Some(artists))
                }
                _ => Err(anyhow!("xesam:artist is not an array.")),
            })
            .unwrap_or(Ok(None))
    }

    /// Metadata mpris:length.
    ///
    /// Returns Err when mpris:length is somehow a different type.
    /// Returns None when mpris:length doesn't exists.
    pub fn length(&self) -> FumResult<Option<Duration>> {
        self.metadata
            .get("mpris:length")
            .map(|length| match length {
                zvariant::Value::I64(length) => Ok(Some(Duration::from_micros(*length as u64))),
                zvariant::Value::U64(length) => Ok(Some(Duration::from_micros(*length))),
                _ => Err(anyhow!("mpris:length is not a i64 / u64.")),
            })
            .unwrap_or(Ok(None))
    }

    /// Metadata mpris:artUrl.
    ///
    /// Returns Err when mpris:artUrl is somehow a different type.
    /// Returns None when mpris:artUrl doesn't exists.
    pub fn art_url(&self) -> FumResult<Option<String>> {
        self.metadata
            .get("mpris:artUrl")
            .map(|art_url| match art_url {
                zvariant::Value::Str(art_url) => Ok(Some(art_url.to_string())),
                _ => Err(anyhow!("mpris:artUrl is not a string.")),
            })
            .unwrap_or(Ok(None))
    }
}
