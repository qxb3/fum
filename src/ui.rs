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
        let chunks: [Rect; 2] =
            Layout::vertical([Constraint::Length(1); 2]).areas(frame.area());

        frame.render_widget(
            Text::from(format!("TrackID: {:?}", track.track_id)),
            chunks[0],
        );

        frame.render_widget(Text::from(format!("Title: {}", track.title)), chunks[1]);
    })?;

    Ok(())
}
