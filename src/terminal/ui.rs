use ratatui::Frame;

use crate::state::State;

/// Render the ui.
pub fn render(state: &mut State, frame: &mut Frame<'_>) {
    let config = state.config();

    frame.render_widget(
        ratatui::text::Text::from(format!("Fps Set: {}. Current Fps: {}", 10, config.fps)),
        frame.area(),
    );
}
