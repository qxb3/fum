use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Paragraph, Widget, Wrap}};

use crate::{get_color, state::FumState};

use super::{Direction, FumWidget};

struct Progress {
    progress_bar: String,
    empty_bar: String,
    progress_area: Rect,
    empty_area: Rect
}

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Progress { direction, progress: prog_opt, empty: empt_opt, .. } = widget {
        let (prog_bg, prog_fg) = get_color!(&prog_opt.bg, &prog_opt.fg, &state.parent_bg, &state.parent_fg);
        let (empt_bg, empt_fg) = get_color!(&empt_opt.bg, &empt_opt.fg, &state.parent_bg, &state.parent_fg);

        let progress_char = prog_opt.char.to_string();
        let empty_char = empt_opt.char.to_string();

        let position = state.meta.position;
        let ratio = if position.as_secs() > 0 {
            position.as_secs() as f64 / state.meta.length.as_secs() as f64
        } else {
            0.0
        };

        let Progress {
            progress_bar,
            empty_bar,
            progress_area,
            empty_area
        } = match direction {
            Direction::Horizontal => {
                let filled = (ratio * area.width as f64).round();
                let empty = area.width.saturating_sub(filled as u16);

                let progress_bar = progress_char.repeat(filled as usize);
                let empty_bar = empty_char.repeat(empty as usize);

                let [progress_area, empty_area] = Layout::horizontal([
                    Constraint::Length(filled as u16),
                    Constraint::Length(empty as u16)
                ]).areas(area);

                Progress {
                    progress_bar,
                    empty_bar,
                    progress_area,
                    empty_area
                }
            },
            Direction::Vertical => {
                let filled = (ratio * area.height as f64).round();
                let empty = area.height.saturating_sub(filled as u16);

                let progress_bar = progress_char.repeat(filled as usize);
                let empty_bar = empty_char.repeat(empty as usize);

                let [empty_area, progress_area] = Layout::vertical([
                    Constraint::Length(empty as u16),
                    Constraint::Length(filled as u16)
                ]).areas(area);

                Progress {
                    progress_bar,
                    empty_bar,
                    progress_area,
                    empty_area
                }
            }
        };

        // Render progress bg
        Block::new()
            .bg(*prog_bg)
            .render(progress_area, buf);

        // Render progress
        Paragraph::new(progress_bar)
            .wrap(Wrap::default())
            .fg(*prog_fg)
            .render(progress_area, buf);

        // Render empty bg
        Block::new()
            .bg(*empt_bg)
            .render(empty_area, buf);

        // Render empty
        Paragraph::new(empty_bar)
            .wrap(Wrap::default())
            .fg(*empt_fg)
            .render(empty_area, buf);
    }
}
