use ratatui::style::Color;
use unicode_width::UnicodeWidthStr;

use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Button() widget function with opts.
pub fn button_opts() -> impl Fn(rhai::Map) -> ScriptFnResult<FumWidget> {
    move |opts: rhai::Map| -> ScriptFnResult<FumWidget> {
        // Extract text from opts.
        let text = opts
            .get("text")
            .cloned()
            .ok_or("Button widget needs to have a `text`")?
            .to_string();

        // Extract func from opts.
        let func = opts
            .get("func")
            .cloned()
            .ok_or("Button widget needs to have a `func`")?
            .try_cast_result::<rhai::FnPtr>()
            .map_err(|_| "Button `func` needs to be a valid function")?;

        // Extract vertical from opts, Will default to false if it doesnt exists.
        let vertical = opts
            .get("vertical")
            .cloned()
            .unwrap_or(rhai::Dynamic::from_bool(false))
            .as_bool()
            .map_err(|_| "Button `vertical` needs to be a boolean")?;

        // Extract fg color from opts, Will default to Color::Reset if it doesnt exists.
        let fg = opts
            .get("fg")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(Color::Reset))
            .try_cast_result::<Color>()
            .map_err(|_| "Label `fg` needs to be a valid color")?;

        // Extract bg color from opts, Will default to Color::Reset if it doesnt exists.
        let bg = opts
            .get("bg")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(Color::Reset))
            .try_cast_result::<Color>()
            .map_err(|_| "Label `bg` needs to be a valid color")?;

        // Get the width & height of the button accordingly if the button is vertical or not.
        let (width, height) = if vertical == false {
            (UnicodeWidthStr::width(text.as_str()) as u16, 1)
        } else {
            (1, UnicodeWidthStr::width(text.as_str()) as u16)
        };

        Ok(FumWidget::Button {
            text: text.to_string(),
            func,
            vertical,
            fg,
            bg,
            width,
            height,
        })
    }
}

/// Button() widget function with default values.
pub fn button() -> impl Fn(rhai::Dynamic, rhai::FnPtr) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic, func: rhai::FnPtr| -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("func".into(), rhai::Dynamic::from(func));
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(false));

        let button_opts = button_opts();
        button_opts(opts)
    }
}

/// Button() widget function with default values & can pass extra opts.
pub fn button_ext_opts(
) -> impl Fn(rhai::Dynamic, rhai::FnPtr, rhai::Map) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic,
          func: rhai::FnPtr,
          ext_opts: rhai::Map|
          -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("func".into(), rhai::Dynamic::from(func));
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(true));
        opts.extend(ext_opts);

        let button_opts = button_opts();
        button_opts(opts)
    }
}

/// ButtonVertical() widget function with vertical opt.
pub fn button_vertical(
) -> impl Fn(rhai::Dynamic, rhai::FnPtr) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic, func: rhai::FnPtr| -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("func".into(), rhai::Dynamic::from(func));
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(true));

        let button_opts = button_opts();
        button_opts(opts)
    }
}

/// ButtonVertical() widget function with vertical opt & can pass extra opts.
pub fn button_vertical_ext_opts(
) -> impl Fn(rhai::Dynamic, rhai::FnPtr, rhai::Map) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic,
          func: rhai::FnPtr,
          ext_opts: rhai::Map|
          -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("func".into(), rhai::Dynamic::from(func));
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(true));
        opts.extend(ext_opts);

        let button_opts = button_opts();
        button_opts(opts)
    }
}
