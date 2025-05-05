use std::ops::Deref;

use ratatui::{
    style::Stylize,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    state::State,
    utils,
    widget::{FumWidget, FumWidgetKind},
};

/// Render the ui.
pub fn render(frame: &mut Frame<'_>, state: &mut State, fps: u64) {
    let layout = state.layout();

    for widget in layout {
        render_widget(&widget, frame);
    }
}

/// Render a widget.
pub fn render_widget(widget: &FumWidget, frame: &mut Frame<'_>) {
    match &widget.kind {
        FumWidgetKind::CoverImage { .. } => {}

        FumWidgetKind::Label {
            text,
            max_chars,
            fg,
            bg,
            ..
        } => {
            let text = match max_chars {
                Some(max_chars) => utils::truncate(&text, *max_chars as usize),
                None => text.to_string(),
            };

            frame.render_widget(
                Paragraph::new(text).wrap(Wrap::default()).fg(*fg).bg(*bg),
                widget.rect,
            );
        }

        FumWidgetKind::Button { widget_kind, .. } => {
            let button_widget = FumWidget::new(widget.rect, widget_kind.deref().clone());
            render_widget(&button_widget, frame);
        }

        FumWidgetKind::Slider { .. } => {}

        _ => {}
    }
}
