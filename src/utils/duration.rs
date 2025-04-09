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
            format!(
                "{:02}:{:02}",
                duration.as_secs() / 60,
                duration.as_secs() % 60
            )
        } else {
            format!("{}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60)
        }
    }
}

/// Calculate and formats the remaining duration.
pub fn format_remaining_duration(
    current: Duration,
    total: Duration,
    extend: bool,
) -> String {
    if total > current {
        let remaining = total - current;
        format!("-{}", format_duration(&remaining, extend))
    } else {
        format!("-0:00")
    }
}
