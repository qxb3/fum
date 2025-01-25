use regex::{Captures, Regex};

use crate::{meta::Meta, utils::etc::format_duration};

pub fn replace_text(text: &str, meta: &Meta) -> String {
    let re = Regex::new(r"get_meta\((.*?)\)").unwrap();

    match text {
        text if text.contains("$title") => text.replace("$title", &meta.title),
        text if text.contains("$artists") => text.replace("$artists", &meta.artists.join(", ")),
        text if text.contains("$album") => text.replace("$album", &meta.album),
        text if text.contains("$status_icon") => text.replace("$status_icon", &meta.status_icon.to_string()),
        text if text.contains("$position") => text.replace("$position", &format_duration(meta.position)),
        text if text.contains("$length") => text.replace("$length", &format_duration(meta.length)),
        text if re.is_match(text) => {
            re.replace_all(text, |c: &Captures| {
                let key = c[1].to_string();
                Meta::get_custom_meta(&meta.metadata, key)
            }).to_string()
        },
        _ => text.to_string()
    }
}
