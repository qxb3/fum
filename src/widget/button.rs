use ratatui::{buffer::Buffer, layout::Rect, widgets::{Paragraph, Widget}};

use crate::text::replace_text;

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Button { text, action, exec, .. } = widget {
        let text = replace_text(text, &state.meta).to_string();

        Paragraph::new(text)
            .render(area, buf);
    }
}
