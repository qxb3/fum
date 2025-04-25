// Text / String Utilities.

/// Truncate a string.
pub fn truncate(str: &str, size: usize) -> String {
    if str.chars().count() <= size {
        return str.to_string();
    }

    let truncated: String = str.chars().take(size).collect();
    format!("{truncated}...")
}
