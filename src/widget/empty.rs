use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize}, widgets::{Block, Widget}};

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, _state: &mut FumWidgetState) {
    if let FumWidget::Empty { bg, fg, .. } = widget {
        Block::new()
            .bg(bg.unwrap_or(Color::Reset))
            .fg(fg.unwrap_or(Color::Reset))
            .render(area, buf);
    }
}
