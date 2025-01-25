use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, text::Text, widgets::{Block, Widget}};

use crate::get_color;

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Progress { progress: prog_opt, empty: empt_opt, .. } = widget {
        let (prog_bg, prog_fg) = get_color!(&prog_opt.bg, &prog_opt.fg, &state.parent_bg, &state.parent_fg);
        let (empt_bg, empt_fg) = get_color!(&empt_opt.bg, &empt_opt.fg, &state.parent_bg, &state.parent_fg);

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

            // Render progress bg
            Block::new()
                .bg(*prog_bg)
                .render(area, buf);

            // Render progress
            Text::from(progress_bar)
                .fg(*prog_fg)
                .render(progress_area, buf);

            // Render empty bg
            Block::new()
                .bg(*empt_bg)
                .render(area, buf);

            // Render empty
            Text::from(empty_bar)
                .fg(*empt_fg)
                .render(empty_area, buf);
        } else {
            // Render empty bg
            Block::new()
                .bg(*empt_bg)
                .render(area, buf);

            Text::from(empty_char.repeat(area.width.into()))
                .fg(*empt_fg)
                .render(area, buf);
        }
    }
}
