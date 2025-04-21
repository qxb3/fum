use unicode_width::UnicodeWidthStr;

use crate::widget::FumWidget;

use super::ScriptFnResult;

/// Label() widget function with opts.
pub fn label_opts() -> impl Fn(rhai::Map) -> ScriptFnResult<FumWidget> {
    move |opts: rhai::Map| -> ScriptFnResult<FumWidget> {
        // Extract text from opts.
        let text = opts
            .get("text")
            .cloned()
            .ok_or("Label widget needs to have a `text`")?
            .to_string();

        // Extract vertical from opts, Will default to false if it doesnt exists.
        let vertical = opts
            .get("vertical")
            .cloned()
            .unwrap_or(rhai::Dynamic::from_bool(false))
            .as_bool()
            .map_err(|_| "Label `vertical` needs to be a boolean")?;

        // Get the width & height of the label accordingly if the label is vertical or not.
        let (width, height) = if vertical == false {
            (UnicodeWidthStr::width(text.as_str()) as u16, 1)
        } else {
            (1, UnicodeWidthStr::width(text.as_str()) as u16)
        };

        Ok(FumWidget::Label {
            text: text.to_string(),
            vertical,
            width,
            height,
        })
    }
}

/// Label() widget function with default opt values.
pub fn label() -> impl Fn(rhai::Dynamic) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic| -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(false));

        let label_opts = label_opts();
        label_opts(opts)
    }
}

/// LabelVertical() widget function with vertical opt.
pub fn label_vertical() -> impl Fn(rhai::Dynamic) -> ScriptFnResult<FumWidget> {
    move |text: rhai::Dynamic| -> ScriptFnResult<FumWidget> {
        let mut opts = rhai::Map::new();
        opts.insert("text".into(), text);
        opts.insert("vertical".into(), rhai::Dynamic::from_bool(true));

        let label_opts = label_opts();
        label_opts(opts)
    }
}
