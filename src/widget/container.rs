use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Margin, Rect}, style::Stylize, widgets::{Block, StatefulWidget, Widget}};

use crate::{get_border, get_color, state::FumState};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Container { width, height, direction, border, padding, children, flex, bg, fg } = widget {
        let area = Rect::new(
            area.x,
            area.y,
            width.unwrap_or(area.width),
            height.unwrap_or(area.height)
        );

        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);
        let border = get_border!(border);

        Block::new()
            .borders(border)
            .bg(*bg)
            .fg(*fg)
            .render(area, buf);

        // Sets the state parents state
        state.parent_direction = direction.to_owned();
        state.parent_bg = *bg;
        state.parent_fg = *fg;

        let areas = Layout::default()
            .direction(direction.to_dir())
            .flex(flex.to_flex())
            .constraints(
                children
                    .iter()
                    .map(|child| child.get_size(state))
                    .collect::<Vec<Constraint>>()
            )
            .split(area);

        for (i, child) in children.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                let [horizontal_padding, vertical_padding] = padding;
                child.render(area.inner(Margin::new(*horizontal_padding, *vertical_padding)), buf, state);
            }
        }
    }
}
