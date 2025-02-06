use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::{get_color, state::FumState};

use super::{Direction, FumWidget};

struct Volume {
    volume_bar: String,
    empty_bar: String,
    volume_area: Rect,
    empty_area: Rect
}

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Volume { direction, volume: vol_opt, empty: empt_opt, .. } = widget {
        let (prog_bg, prog_fg) = get_color!(&vol_opt.bg, &vol_opt.fg, &state.parent_bg, &state.parent_fg);
        let (empt_bg, empt_fg) = get_color!(&empt_opt.bg, &empt_opt.fg, &state.parent_bg, &state.parent_fg);

        let progress_char = vol_opt.char.to_string();
        let empty_char = empt_opt.char.to_string();

        let volume = match direction {
            Direction::Horizontal => {
                let filled = (state.meta.volume * area.width as f64).round();
                let empty = area.width.saturating_sub(filled as u16);

                let volume_bar = progress_char.repeat(filled as usize);
                let empty_bar = empty_char.repeat(empty as usize);

                let [volume_area, empty_area] = Layout::horizontal([
                    Constraint::Length(filled as u16),
                    Constraint::Length(empty as u16)
                ]).areas(area);

                Volume {
                    volume_bar,
                    empty_bar,
                    volume_area,
                    empty_area
                }
            },
            Direction::Vertical => {
                let filled = (state.meta.volume * area.height as f64).round();
                let empty = area.height.saturating_sub(filled as u16);

                let volume_bar = progress_char.repeat(filled as usize);
                let empty_bar = empty_char.repeat(empty as usize);

                let [empty_area, volume_area] = Layout::vertical([
                    Constraint::Length(empty as u16),
                    Constraint::Length(filled as u16)
                ]).areas(area);

                Volume {
                    volume_bar,
                    empty_bar,
                    volume_area,
                    empty_area
                }
            }
        };

        // Render volume filled bg
        Block::new()
            .bg(*prog_bg)
            .render(volume.volume_area, buf);

        // Render volume filled
        Paragraph::new(volume.volume_bar)
            .wrap(Wrap::default())
            .fg(*prog_fg)
            .render(volume.volume_area, buf);

        // Render empty bg
        Block::new()
            .bg(*empt_bg)
            .render(volume.empty_area, buf);

        // Render empty
        Paragraph::new(volume.empty_bar)
            .wrap(Wrap::default())
            .fg(*empt_fg)
            .render(volume.empty_area, buf);
    }
}
