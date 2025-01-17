use std::io::stdout;

use crossterm::{event::DisableMouseCapture, execute};

#[macro_export]
macro_rules! debug_widget {
    ($frame:expr, $area:expr) => {
        $frame.render_widget(
            ratatui::widgets::Block::new()
                .borders(ratatui::widgets::Borders::ALL),
            $area
        )
    };
}

#[macro_export]
macro_rules! config_debug {
    ($debug:expr, $frame:expr, $area:expr) => {
        if let Some(debug) = $debug {
            if debug {
                debug_widget!($frame, $area);
            }
        }
    };
}

pub fn restore() {
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)
        .expect("Failed to disable mouse capture.");
}
