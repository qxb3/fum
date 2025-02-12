use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::{get_bold, get_color, state::FumState, text::replace_text};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Button { id, text, action, exec, bold, bg, fg, .. } = widget {
        let text = replace_text(text, state).to_string();

        state.buttons.insert(
            id.to_string(),
            (area.clone(), action.to_owned(), exec.to_owned())
        );

        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);
        let bold = get_bold!(bold);

        // Render bg
        Block::new()
            .bg(*bg)
            .render(area, buf);

        Paragraph::new(text)
            .wrap(Wrap::default())
            .add_modifier(bold)
            .fg(*fg)
            .render(area, buf);
    }
}
