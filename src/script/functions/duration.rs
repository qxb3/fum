use std::time::Duration;

use crate::utils::duration::format_duration;

/// .fmt() type function for Duration.
/// Formats the Duration into human readable string.
pub fn duration_fmt() -> impl Fn(&mut Duration) -> String {
    move |duration: &mut Duration| -> String {
        format_duration(duration, false)
    }
}

/// .fmt_ext() type function for Duration.
/// Formats the Duration into human readable string with 0 extension.
pub fn duration_fmt_ext() -> impl Fn(&mut Duration) -> String {
    move |duration: &mut Duration| -> String {
        format_duration(duration, true)
    }
}
