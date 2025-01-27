use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, Paragraph, Widget}};

use crate::{get_color, state::FumState, text::replace_text, utils};

use super::{FumWidget, LabelAlignment};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Label { text, truncate, align, bg, fg } = widget {
        let text = match truncate {
            true => utils::etc::truncate(&replace_text(text, state), area.width.into()),
            false => replace_text(text, state)
        };

        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        let widget = match align {
            LabelAlignment::Left => Paragraph::new(text).left_aligned().fg(*fg),
            LabelAlignment::Center => Paragraph::new(text).centered().fg(*fg),
            LabelAlignment::Right => Paragraph::new(text).right_aligned().fg(*fg),
        };

        // Render bg
        Block::new()
            .bg(*bg)
            .render(area, buf);

        widget.render(area, buf);
    }
}
