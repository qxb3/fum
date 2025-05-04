use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::FumErr;

/// Render the error ui.
pub fn render(err: &FumErr, frame: &mut Frame<'_>) {
    let width = frame.area().width / 2;
    let height = frame.area().height / 2;

    // Center horizontally.
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(frame.area());

    // Center vertically.
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);

    // Render the error box.
    frame.render_widget(
        Paragraph::new(err.to_string()).wrap(Wrap::default()).block(
            Block::new()
                .title("⚠️ ERROR ⚠️")
                .borders(Borders::all())
                .fg(Color::Rgb(179, 36, 36)),
        ),
        area,
    );
}
