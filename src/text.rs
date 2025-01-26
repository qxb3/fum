use regex::{Captures, Regex};

use crate::{meta::Meta, state::FumState, utils::etc::{format_duration, format_remaining}};

pub fn replace_text(text: &str, state: &mut FumState) -> String {
    let get_meta_re = Regex::new(r"get_meta\((.*?)\)").unwrap();
    let var_re = Regex::new(r"var\((\$\w+),\s*(\$\w+)\)").unwrap();

    match text {
        text if get_meta_re.is_match(text) => {
            get_meta_re.replace_all(text, |c: &Captures| {
                let key = c[1].to_string();
                Meta::get_custom_meta(&state.meta.metadata, key)
            }).to_string()
        },

        text if var_re.is_match(text) => {
            var_re.replace_all(text, |c: &Captures| {
                let mut vars = state.vars.clone();

                let name = c[1].to_string();
                let default_text = c[2].to_string();

                match vars.get(&name) {
                    Some(var) => return replace_text(var, state),
                    None => {
                        vars.insert(name, default_text.to_string());

                        // Update state.vars
                        state.vars = vars;

                        return replace_text(&default_text, state);
                    }
                }
            }).to_string()
        },

        text if text.contains("$title") => text.replace("$title", &state.meta.title),
        text if text.contains("$artists") => text.replace("$artists", &state.meta.artists.join(", ")),
        text if text.contains("$album") => text.replace("$album", &state.meta.album),
        text if text.contains("$status_icon") => text.replace("$status_icon", &state.meta.status_icon.to_string()),
        text if text.contains("$position") => text.replace("$position", &format_duration(state.meta.position)),
        text if text.contains("$remaining-length") => text.replace("$remaining-length", &format_remaining(state.meta.position, state.meta.length)),
        text if text.contains("$length") => text.replace("$length", &format_duration(state.meta.length)),

        _ => text.to_string()
    }
}
