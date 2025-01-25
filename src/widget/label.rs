use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

use crate::{text::replace_text, utils};

use super::{FumWidget, FumWidgetState, LabelAlignment};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Label { text, truncate, align, bg, fg } = widget {
        let text = match truncate {
            true => utils::etc::truncate(&replace_text(text, &state.meta), area.width.into()),
            false => replace_text(text, &state.meta)
        };

        let widget = match align {
            LabelAlignment::Left => Paragraph::new(text).left_aligned()/* .fg(*fg) */,
            LabelAlignment::Center => Paragraph::new(text).centered()/* .fg(*fg) */,
            LabelAlignment::Right => Paragraph::new(text).right_aligned()/* .fg(*fg) */,
        };

        widget.render(area, buf);
    }
}
