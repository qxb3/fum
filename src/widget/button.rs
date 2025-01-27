use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, Paragraph, Widget}};

use crate::{get_color, state::FumState, text::replace_text};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Button { id, text, action, exec, bg, fg } = widget {
        let text = replace_text(text, state).to_string();

        state.buttons.insert(
            id.to_string(),
            (area.clone(), action.to_owned(), exec.to_owned())
        );

        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        // Render bg
        Block::new()
            .bg(*bg)
            .render(area, buf);

        Paragraph::new(text)
            .fg(*fg)
            .render(area, buf);
    }
}
