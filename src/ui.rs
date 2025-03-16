use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    text::Text,
    Terminal,
};
use ratatui_image::StatefulImage;

use crate::{state::State, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &State,
) -> FumResult<()> {
    let player = state.current_player.lock().await;
    let track = state.current_track.lock().await;
    let mut cover = state.current_cover.lock().await;

    // Drawing part.
    terminal.draw(|frame| {
        let split_chunks: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(frame.area());

        if let Some(cover) = cover.as_mut() {
            frame.render_stateful_widget(
                StatefulImage::default(),
                split_chunks[0],
                cover,
            );
        }

        let meta_chunks: [Rect; 11] =
            Layout::vertical([Constraint::Length(1); 11]).areas(split_chunks[1]);

        frame.render_widget(
            Text::from(format!("TrackID: {:?}", track.track_id)),
            meta_chunks[0],
        );

        frame.render_widget(
            Text::from(format!("Title: {}", track.title)),
            meta_chunks[1],
        );

        frame.render_widget(
            Text::from(format!("Album: {}", track.album)),
            meta_chunks[2],
        );

        frame.render_widget(
            Text::from(format!("Artists: {:?}", track.artists)),
            meta_chunks[3],
        );

        frame.render_widget(
            Text::from(format!("Length: {}", track.length.as_secs())),
            meta_chunks[4],
        );

        frame.render_widget(
            Text::from(format!("Art url: {:?}", track.art_url)),
            meta_chunks[5],
        );

        frame.render_widget(
            Text::from(format!("Playback: {:?}", track.playback_status)),
            meta_chunks[6],
        );

        frame.render_widget(
            Text::from(format!("shuffle: {}", track.shuffle)),
            meta_chunks[7],
        );

        frame.render_widget(
            Text::from(format!("volume: {}", track.volume)),
            meta_chunks[8],
        );

        frame.render_widget(
            Text::from(format!("position: {}", track.position.as_secs())),
            meta_chunks[9],
        );

        if let Some(player) = player.as_ref() {
            frame.render_widget(
                Text::from(format!("player: {}", player.identity)),
                meta_chunks[10],
            );
        }
    })?;

    Ok(())
}
