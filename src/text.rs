use regex::Captures;

use crate::{meta::Meta, regexes::{GET_META_RE, LOWER_RE, UPPER_RE, VAR_RE}, state::FumState, utils::widget::{format_duration, format_remaining}};

fn replace_global_var(text: &str, state: &mut FumState) -> String {
    match text {
        text if text.contains("$title")                 => text.replace("$title", &state.meta.title),
        text if text.contains("$artists")               => text.replace("$artists", &state.meta.artists.join(", ")),
        text if text.contains("$album")                 => text.replace("$album", &state.meta.album),

        text if text.contains("$status-icon")           => text.replace("$status-icon", &state.meta.status_icon.to_string()),
        text if text.contains("$status-text")           => text.replace("$status-text", &state.meta.status_text),

        text if text.contains("$position-ext")          => text.replace("$position-ext", &format_duration(state.meta.position, true)),
        text if text.contains("$position")              => text.replace("$position", &format_duration(state.meta.position, false)),

        text if text.contains("$remaining-length-ext")  => text.replace("$remaining-length-ext", &format_remaining(state.meta.position, state.meta.length, true)),
        text if text.contains("$remaining-length")      => text.replace("$remaining-length", &format_remaining(state.meta.position, state.meta.length, false)),

        text if text.contains("$length-ext")            => text.replace("$length-ext", &format_duration(state.meta.length, true)),
        text if text.contains("$length")                => text.replace("$length", &format_duration(state.meta.length, false)),

        text if text.contains("$volume")                => text.replace("$volume", &format!("{:.0}", state.meta.volume * 100.0)),
        _                                               => text.to_string()
    }
}

pub fn replace_text(text: &str, state: &mut FumState) -> String {
    match text {
        // get_meta() text action
        text if GET_META_RE.is_match(text) => {
            GET_META_RE.replace_all(text, |c: &Captures| {
                let key = c[1].to_string();
                Meta::get_custom_meta(&state.meta.metadata, key)
            }).to_string()
        },

        // var() text action
        text if VAR_RE.is_match(text) => {
            VAR_RE.replace_all(text, |c: &Captures| {
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

        // lower() text action
        text if LOWER_RE.is_match(text) => {
            LOWER_RE.replace_all(text, |c: &Captures| {
                let value = c[1].to_string();

                if value.starts_with("$") {
                    replace_global_var(&value, state).to_lowercase()
                } else {
                    value.to_lowercase()
                }
            }).to_string()
        },

        // upper() text action
        text if UPPER_RE.is_match(text) => {
            UPPER_RE.replace_all(text, |c: &Captures| {
                let value = c[1].to_string();

                if value.starts_with("$") {
                    replace_global_var(&value, state).to_uppercase()
                } else {
                    value.to_uppercase()
                }
            }).to_string()
        },

        // global vars
        text if text.contains("$") => replace_global_var(text, state),

        _ => text.to_string()
    }
}
