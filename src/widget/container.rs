use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::StatefulWidget};

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::Container { width, height, direction, children, flex, bg, fg } = widget {
        let area = Rect::new(
            area.x,
            area.y,
            width.unwrap_or(area.width),
            height.unwrap_or(area.height)
        );

        let areas = Layout::default()
            .direction(direction.to_dir())
            .flex(flex.to_flex())
            .constraints(
                children
                    .iter()
                    .map(|child| child.get_size(direction))
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
