use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    text::Text,
    Terminal,
};

use crate::{state::State, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &State,
) -> FumResult<()> {
    // Get current track.
    let track = state.current_track.lock().await;

    // Drawing part.
    terminal.draw(|frame| {
        let chunks: [Rect; 10] =
            Layout::vertical([Constraint::Length(1); 10]).areas(frame.area());

        frame.render_widget(
            Text::from(format!("TrackID: {:?}", track.track_id)),
            chunks[0],
        );

        frame.render_widget(Text::from(format!("Title: {}", track.title)), chunks[1]);

        frame.render_widget(Text::from(format!("Album: {}", track.album)), chunks[2]);

        frame.render_widget(
            Text::from(format!("Artists: {:?}", track.artists)),
            chunks[3],
        );

        frame.render_widget(
            Text::from(format!("Length: {}", track.length.as_secs())),
            chunks[4],
        );

        frame.render_widget(
            Text::from(format!("Art url: {:?}", track.art_url)),
            chunks[5],
        );

        frame.render_widget(
            Text::from(format!("Playback: {:?}", track.playback_status)),
            chunks[6],
        );

        frame.render_widget(Text::from(format!("shuffle: {}", track.shuffle)), chunks[7]);

        frame.render_widget(Text::from(format!("volume: {}", track.volume)), chunks[8]);

        frame.render_widget(
            Text::from(format!("position: {}", track.position.as_secs())),
            chunks[9],
        );
    })?;

    Ok(())
}
