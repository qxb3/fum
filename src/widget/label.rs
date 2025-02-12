use ratatui::{buffer::Buffer, layout::Rect, style::{Modifier, Stylize}, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::{get_bold, get_color, state::FumState, text::replace_text, utils};

use super::{Direction, FumWidget, LabelAlignment};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Label { text, direction, truncate, align, bold, bg, fg } = widget {
        let text = match (truncate, direction) {
            (true, Direction::Horizontal) => utils::etc::truncate(&replace_text(text, state), area.width.into()),
            (true, Direction::Vertical) => utils::etc::truncate(&replace_text(text, state), area.height.into()),
            _ => replace_text(text, state)
        };

        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        // Whether the text is bold
        let bold = get_bold!(bold);

        let widget = match align {
            LabelAlignment::Left => Paragraph::new(text).left_aligned().wrap(Wrap::default()).fg(*fg).add_modifier(bold),
            LabelAlignment::Center => Paragraph::new(text).centered().wrap(Wrap::default()).fg(*fg).add_modifier(bold),
            LabelAlignment::Right => Paragraph::new(text).right_aligned().wrap(Wrap::default()).fg(*fg).add_modifier(bold),
        };

        // Render bg
        Block::new()
            .bg(*bg)
            .render(area, buf);

        widget.render(area, buf);
    }
}
