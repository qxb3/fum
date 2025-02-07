use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::{Block, Borders, StatefulWidget, Widget}};

use crate::{get_color, state::FumState};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::Container { width, height, direction, border, children, flex, bg, fg } = widget {
        let area = Rect::new(
            area.x,
            area.y,
            width.unwrap_or(area.width),
            height.unwrap_or(area.height)
        );

        // Render this container block with defined or parent's bg / fg
        let (bg, fg) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        // Whether to render border
        let border = match border {
            true => Borders::ALL,
            false => Borders::NONE
        };

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
                child.render(*area, buf, state);
            }
        }
    }
}
