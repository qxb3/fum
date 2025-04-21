use ratatui::style::Color;

use crate::widget::{FumWidget, SliderDataSource, SliderTextOpts};

use super::ScriptFnResult;

/// SLIDER() widget function with opts.
pub fn slider_opts() -> impl Fn(rhai::Map) -> ScriptFnResult<FumWidget> {
    move |opts: rhai::Map| -> ScriptFnResult<FumWidget> {
        // Extract data source from opts.
        let data_source = opts
            .get("source")
            .cloned()
            .ok_or("Slider widget needs to have a `source`")?
            .try_cast_result::<SliderDataSource>()
            .map_err(|_| {
                "Slider `source` needs to be either SOURCE_PROGRESS or SOURCE_VOLUME"
            })?;

        // Extract filled from opts.
        let filled_opts = opts
            .get("filled")
            .cloned()
            .ok_or("Slider widget needs to have a `filled`")?
            .try_cast_result::<rhai::Map>()
            .map_err(|_| "Slider `filled` needs to be a map")?;

        // Extract remaining from opts.
        let remaining_opts = opts
            .get("remaining")
            .cloned()
            .ok_or("Slider widget needs to have a `remaining`")?
            .try_cast_result::<rhai::Map>()
            .map_err(|_| "Slider `remaining` needs to be a map")?;

        let filled = get_slider_text_opts(filled_opts)?;
        let remaining = get_slider_text_opts(remaining_opts)?;

        Ok(FumWidget::Slider {
            data_source,
            filled,
            remaining,
        })
    }
}

/// A helper function to extract text opts from map.
fn get_slider_text_opts(opts: rhai::Map) -> ScriptFnResult<SliderTextOpts> {
    // Extract text from text opts.
    let text = opts
        .get("text")
        .cloned()
        .ok_or("Slider widget needs to have `filled.text` or `remaining.text`")?
        .to_string();

    // Extract fg from text opts, Will default to Color::Reset if it doesnt exists.
    let fg = opts
        .get("fg")
        .cloned()
        .unwrap_or(rhai::Dynamic::from(Color::Reset))
        .try_cast_result::<Color>()
        .map_err(|_| "Slider `filled.fg` or `remaining.fg` needs to be a valid color")?;

    // Extract bg from text opts, Will default to Color::Reset if it doesnt exists.
    let bg = opts
        .get("bg")
        .cloned()
        .unwrap_or(rhai::Dynamic::from(Color::Reset))
        .try_cast_result::<Color>()
        .map_err(|_| "Slider `filled.bg` or `remaining.bg` needs to be a valid color")?;

    Ok(SliderTextOpts { text, fg, bg })
}
