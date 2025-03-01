use std::{error::Error, time::Duration};

use mpris::Mpris;

mod mpris;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    let mpris = Mpris::new().await?;

    let players = mpris.players().await?;

    if let Some(spotify) = players.get("org.mpris.MediaPlayer2.spotify") {
        let metadata = spotify.metadata().await?;

        let trackid = metadata.trackid()?.unwrap_or("No trackid".into());
        let title = metadata.title()?.unwrap_or("No Title".into());
        let album = metadata.album()?.unwrap_or("No Album".into());
        let artists = metadata.artists()?.unwrap_or(vec!["No Artists".into()]);
        let length = metadata.length()?.unwrap_or(Duration::from_secs_f32(99.0));
        let art_url = metadata.art_url()?.unwrap_or("No art url".into());

        println!("trackid: {trackid}");
        println!("title: {title}");
        println!("album: {album}");
        println!("artists: {:?}", artists);
        println!("length: {}", length.as_secs());
        println!("art url: {art_url}");
    }

    Ok(())
}
