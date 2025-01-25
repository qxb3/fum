use std::io::stdout;

use crossterm::{event::DisableMouseCapture, execute};

pub fn restore() {
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)
        .expect("Failed to disable mouse capture.");
}
