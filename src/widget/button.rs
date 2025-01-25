use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

use crate::text::replace_text;

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Button { id, text, action, exec, .. } = widget {
        let text = replace_text(text, &state.meta).to_string();

        state.buttons.insert(
            id.to_string(),
            (area.clone(), action.to_owned(), exec.to_owned())
        );

        Paragraph::new(text)
            .render(area, buf);
    }
}
