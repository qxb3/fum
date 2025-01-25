use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Stylize}, text::Text, widgets::Widget};

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Progress { progress: prog_opt, empty: empt_opt, .. } = widget {
        let progress_char = prog_opt.char.to_string();
        let empty_char = empt_opt.char.to_string();

        if state.meta.length.as_secs() > 0 {
            let ratio = state.meta.position.as_secs() as f64 / state.meta.length.as_secs() as f64;

            let filled = (ratio * area.width as f64).round();
            let empty = area.width.saturating_sub(filled as u16);

            let progress_bar = progress_char.repeat(filled as usize);
            let empty_bar = empty_char.repeat(empty.into());

            let [progress_area, empty_area] = Layout::horizontal([
                Constraint::Length(filled as u16),
                Constraint::Length(empty as u16),
            ]).areas(area);

            Text::from(progress_bar)
                .fg(prog_opt.fg.unwrap_or(Color::Reset))
                .render(progress_area, buf);

            Text::from(empty_bar)
                .fg(empt_opt.fg.unwrap_or(Color::Reset))
                .render(empty_area, buf);
        } else {
            Text::from(empty_char.repeat(area.width.into()))
                .fg(empt_opt.fg.unwrap_or(Color::Reset))
                .render(area, buf);
        }
    }
}
