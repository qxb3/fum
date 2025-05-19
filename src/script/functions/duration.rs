use std::time::Duration;

use crate::utils;

/// <Duration>.is_millis().
pub fn duration_as_millis() -> impl Fn(&mut Duration) -> rhai::INT {
    move |duration: &mut Duration| -> rhai::INT { duration.as_millis() as i64 }
}

/// <Duration>.is_nanos().
pub fn duration_as_nanos() -> impl Fn(&mut Duration) -> rhai::INT {
    move |duration: &mut Duration| -> rhai::INT { duration.as_nanos() as i64 }
}

/// <Duration>.is_secs().
pub fn duration_as_secs() -> impl Fn(&mut Duration) -> rhai::INT {
    move |duration: &mut Duration| -> rhai::INT { duration.as_secs() as i64 }
}

/// <Duration>.is_zero().
pub fn duration_is_zero() -> impl Fn(&mut Duration) -> bool {
    move |duration: &mut Duration| -> bool { duration.is_zero() }
}

/// <Duration>.fmt().
pub fn duration_fmt() -> impl Fn(&mut Duration) -> String {
    move |duration: &mut Duration| -> String { utils::format_duration(duration, false) }
}

/// <Duration>.fmt_ext().
pub fn duration_fmt_ext() -> impl Fn(&mut Duration) -> String {
    move |duration: &mut Duration| -> String { utils::format_duration(duration, true) }
}
