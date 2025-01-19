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

pub fn generate_btn_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn truncate(string: &str, width: usize) -> String {
    if string.chars().count() <= width {
        string.to_string()
    } else {
        // minus 3 since the dots (...)
        let truncated: String = string.chars().take(width - 3).collect();
        format!("{}...", truncated)
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration.as_secs() >= 3600 {
        format!(
            "{}:{:02}:{:02}",
            duration.as_secs() / 3600,
            (duration.as_secs() % 3600) / 60,
            duration.as_secs() % 60
        )
    } else {
        format!("{}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
    }
}
