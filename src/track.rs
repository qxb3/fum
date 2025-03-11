/// Contains the metadata of the current track / song.
#[derive(Debug)]
pub struct Track {
    pub track_id: Option<String>,
    pub title: String,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            track_id: None,
            title: "No Music".into(),
        }
    }
}

impl Track {
    /// Creates a new track.
    pub fn new() -> Self {
        Self::default()
    }
}
