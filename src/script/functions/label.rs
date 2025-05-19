use anyhow::anyhow;
use ratatui::style::Color;
use unicode_width::UnicodeWidthStr;

use crate::{event::EventSender, widget::FumWidgetKind};

/// raw function of LABEL().
pub fn label_function_raw(
    event_sender: EventSender,
) -> impl Fn(rhai::Map) -> Option<FumWidgetKind> {
    move |opts: rhai::Map| -> Option<FumWidgetKind> {
        // Extract text from opts.
        let text = match opts.get("text").cloned() {
            Some(text_opt) => text_opt.to_string(),
            None => {
                event_sender
                    .send(Err(anyhow!("Label widget needs to have a `text`")))
                    .unwrap();

                return None;
            }
        };

        // Extract vertical from opts, Will default to false if it doesnt exists.
        let vertical = match opts
            .get("vertical")
            .cloned()
            .unwrap_or(rhai::Dynamic::from_bool(false))
            .as_bool()
        {
            Ok(vertical) => vertical,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Label `vertical` needs to be a boolean")))
                    .unwrap();

                return None;
            }
        };

        // Extract max_chars from opts, Will default to None if it doesnt exists.
        let max_chars = match opts.get("max_chars").cloned() {
            Some(max_chars) => match max_chars.as_int() {
                Ok(max_chars) => Some(max_chars as u16),
                Err(_) => {
                    event_sender
                        .send(Err(anyhow!("Label `max_chars` needs to be a valid number")))
                        .unwrap();

                    return None;
                }
            },
            None => None,
        };

        // Extract fg color from opts, Will default to Color::Reset if it doesnt exists.
        let fg = match opts
            .get("fg")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(Color::Reset))
            .try_cast_result::<Color>()
        {
            Ok(fg) => fg,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Label `fg` needs to be a valid color")))
                    .unwrap();

                return None;
            }
        };

        // Extract bg color from opts, Will default to Color::Reset if it doesnt exists.
        let bg = match opts
            .get("bg")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(Color::Reset))
            .try_cast_result::<Color>()
        {
            Ok(bg) => bg,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Label `bg` needs to be a valid color")))
                    .unwrap();

                return None;
            }
        };

        // Real unicode width of the text label.
        let real_unicode_width = UnicodeWidthStr::width(text.as_str()) as u16;

        // If the max_chars isnt set, use the real unicode width, else use the max_chars if its less than the real unicode width.
        let unicode_width = match max_chars {
            Some(max_chars) => ((max_chars + 3) as u16).min(real_unicode_width), // +3 to account for ellipses (...).
            None => real_unicode_width,
        };

        // Get the width & height of the label accordingly if the label is vertical or not.
        let (width, height) = match vertical {
            true => (1, unicode_width),
            false => (unicode_width, 1),
        };

        Some(FumWidgetKind::Label {
            text: text.to_string(),
            vertical,
            max_chars,
            fg,
            bg,
            width,
            height,
        })
    }
}

/// LABEL() a wrapper around raw.
pub fn label_function(
    event_sender: EventSender,
) -> impl Fn(rhai::Dynamic) -> Option<FumWidgetKind> {
    move |text: rhai::Dynamic| -> Option<FumWidgetKind> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);

        let raw = label_function_raw(event_sender.clone());
        raw(opts)
    }
}

/// LABEL() a wrapper around raw & can pass extra opts.
pub fn label_function_ext(
    event_sender: EventSender,
) -> impl Fn(rhai::Dynamic, rhai::Map) -> Option<FumWidgetKind> {
    move |text: rhai::Dynamic, ext_opts: rhai::Map| -> Option<FumWidgetKind> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.extend(ext_opts);

        let raw = label_function_raw(event_sender.clone());
        raw(opts)
    }
}
