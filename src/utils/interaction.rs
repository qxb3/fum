// Ui Interaction Utilities.

use ratatui::layout::{Position, Rect};

use crate::{
    script::ScriptUi,
    widget::{FumWidget, SliderDataSource},
    FumResult,
};

// Returns the interacted slider widget rect & slider source if interacted.
pub fn get_interacted_slider(
    ui: ScriptUi,
    start_drag: &Position,
) -> FumResult<Option<(Rect, SliderDataSource)>> {
    let ui = ui
        .lock()
        .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;

    for (rect, widget) in &*ui {
        match widget {
            // Only check for slider widgets.
            FumWidget::Slider { data_source, .. } => {
                if rect.contains(*start_drag) {
                    return Ok(Some((*rect, *data_source)));
                }
            }

            _ => {}
        }
    }

    Ok(None)
}
