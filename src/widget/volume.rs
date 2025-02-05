use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, text::Text, widgets::{Block, Widget}};

use crate::{get_color, state::FumState};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Volume { volume: vol_opt, empty: empt_opt, .. } = widget {
        let (prog_bg, prog_fg) = get_color!(&vol_opt.bg, &vol_opt.fg, &state.parent_bg, &state.parent_fg);
        let (empt_bg, empt_fg) = get_color!(&empt_opt.bg, &empt_opt.fg, &state.parent_bg, &state.parent_fg);

        let progress_char = vol_opt.char.to_string();
        let empty_char = empt_opt.char.to_string();

        let filled = (state.meta.volume * area.width as f64).round();
        let empty = area.width.saturating_sub(filled as u16);

        let volume_bar = progress_char.repeat(filled as usize);
        let empty_bar = empty_char.repeat(empty as usize);

        let [progress_area, empty_area] = Layout::horizontal([
            Constraint::Length(filled as u16),
            Constraint::Length(empty as u16),
        ]).areas(area);

        // Render volume filled bg
        Block::new()
            .bg(*prog_bg)
            .render(area, buf);

        // Render volume filled
        Text::from(volume_bar)
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
    }
}
