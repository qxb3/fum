use std::{collections::HashMap, time::Duration};

use zbus::zvariant;

use crate::FumResult;

/// Type alias for zvariant::Value<'_>.
pub type MetadataValue<'a> = zvariant::Value<'a>;

#[derive(Debug)]
pub struct Metadata<'a> {
    metadata: HashMap<String, MetadataValue<'a>>,
}

impl<'a> Metadata<'a> {
    /// Creates a new Metadata.
    pub fn new(metadata: HashMap<String, MetadataValue<'a>>) -> FumResult<Self> {
        Ok(Self {
            metadata,
        })
    }

    /// Metadata mpris:trackid.
    ///
    /// Returns Err when mpris:trackid is somehow a different type.
    /// Returns None when mpris:trackid doesn't exists.
    pub fn trackid(&self) -> FumResult<Option<String>> {
        self.metadata
            .get("mpris:trackid")
            .map(|trackid| match trackid {
                MetadataValue::Str(trackid) => Ok(Some(trackid.to_string())),
                _ => Err("mpris:trackid is not a string.".into()),
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
                MetadataValue::Str(title) => Ok(Some(title.to_string())),
                _ => Err("xesam:title is not a string.".into()),
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
                MetadataValue::Str(album) => Ok(Some(album.to_string())),
                _ => Err("xesam:album is not a string.".into()),
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
                MetadataValue::Array(artists) => {
                    let artists: Vec<String> = artists
                        .iter()
                        .filter_map(|a| {
                            a.downcast_ref::<&str>().map(|s| s.to_string()).ok()
                        })
                        .collect();

                    Ok(Some(artists))
                }
                _ => Err("xesam:artist is not an array.".into()),
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
                MetadataValue::U64(length) => Ok(Some(Duration::from_micros(*length))),
                _ => Err("mpris:length is not a string.".into()),
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
                MetadataValue::Str(art_url) => Ok(Some(art_url.to_string())),
                _ => Err("mpris:artUrl is not a string.".into()),
            })
            .unwrap_or(Ok(None))
    }
}
