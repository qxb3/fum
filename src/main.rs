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
        let length = metadata
            .length()?
            .unwrap_or(Duration::from_secs_f32(9999.0));
        let art_url = metadata.art_url()?.unwrap_or("No art url".into());

        let playback_status = spotify.playback_status().await?;
        let loop_status = spotify.loop_status().await?;
        let playback_rate = spotify.playback_rate().await?;
        let min_playback_rate = spotify.min_playback_rate().await?;
        let max_playback_rate = spotify.max_playback_rate().await?;
        let shuffle = spotify.shuffle().await?;
        let volume = spotify.volume().await?;
        let position = spotify.position().await?;
        let can_next = spotify.can_next().await?;
        let can_prev = spotify.can_previous().await?;
        let can_play = spotify.can_play().await?;
        let can_pause = spotify.can_pause().await?;
        let can_seek = spotify.can_seek().await?;
        let can_control = spotify.can_control().await?;

        println!("trackid: {trackid}");
        println!("title: {title}");
        println!("album: {album}");
        println!("artists: {:?}", artists);
        println!("length: {}", length.as_secs());
        println!("art url: {art_url}");

        println!("playback status: {:?}", playback_status);
        println!("loop status: {:?}", loop_status);
        println!("playback rate: {playback_rate}");
        println!("min playback rate: {min_playback_rate}");
        println!("max playback rate: {max_playback_rate}");
        println!("shuffle: {shuffle}");
        println!("volume: {volume}");
        println!("position: {}", position.as_secs());
        println!("can next: {can_next}");
        println!("can prev: {can_prev}");
        println!("can play: {can_play}");
        println!("can pause: {can_pause}");
        println!("can seek: {can_seek}");
        println!("can control: {can_control}");
    }

    Ok(())
}
