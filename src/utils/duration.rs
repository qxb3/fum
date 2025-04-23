use std::time::Duration;

/// Formats duration into a readable string format.
pub fn format_duration(duration: &Duration, extend: bool) -> String {
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
