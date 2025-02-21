use std::time::Duration;
use uuid::Uuid;

#[macro_export]
macro_rules! get_size {
    ($orientation:expr, $size:expr, $area:expr) => {{
        let [area] = match $size {
            Some(width) => $orientation([Constraint::Length(*width)]).areas($area),
            None => $orientation([Constraint::Min(0)]).areas($area),
        };

        area
    }};
}

#[macro_export]
macro_rules! get_color {
    ($bg:expr, $fg:expr, $parent_bg:expr, $parent_fg:expr) => {{
        let bg = match $bg {
            Some(bg) => bg,
            None => $parent_bg,
        };

        let fg = match $fg {
            Some(fg) => fg,
            None => $parent_fg,
        };

        (bg, fg)
    }};
}

#[macro_export]
macro_rules! get_bold {
    ($bold:expr) => {{
        match $bold {
            true => ratatui::style::Modifier::BOLD,
            false => ratatui::style::Modifier::default()
        }
    }};
}

#[macro_export]
macro_rules! get_border {
    ($border:expr) => {{
        match $border {
            true => ratatui::widgets::Borders::ALL,
            false => ratatui::widgets::Borders::NONE
        }
    }};
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn truncate(string: &str, area_size: usize) -> String {
    if string.chars().count() <= area_size {
        string.to_string()
    } else {
        // minus 3 since the dots (...)
        let take = if area_size > 3 { area_size - 3 } else { area_size };
        let truncated: String = string.chars().take(take).collect();
        format!("{}...", truncated)
    }
}

pub fn format_duration(duration: Duration, extend: bool) -> String {
    if duration.as_secs() >= 3600 {
        if extend {
            format!(
                "{:02}:{:02}:{:02}",
                duration.as_secs() / 3600,
                (duration.as_secs() % 3600) / 60,
                duration.as_secs() % 60
            )
        } else {
            format!(
                "{}:{:02}:{:02}",
                duration.as_secs() / 3600,
                (duration.as_secs() % 3600) / 60,
                duration.as_secs() % 60
            )
        }
    } else {
        if extend {
            format!("{:02}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
        } else {
            format!("{}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
        }
    }
}

pub fn format_remaining(current: Duration, total: Duration, extend: bool) -> String {
    if total > current {
        let remaining = total - current;
        format!("-{}", format_duration(remaining, extend))
    } else {
        format!("-0:00")
    }
}

