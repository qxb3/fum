use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::{Block, StatefulWidget, Widget}};
use ratatui_image::StatefulImage;

use crate::get_color;

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::CoverArt { resize, bg, fg, .. } = widget {
        let (bg, _) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        // Render bg
        Block::new()
            .bg(*bg)
            .render(area, buf);

        if let Some(cover_art) = state.meta.cover_art.as_mut() {
            StatefulWidget::render(
                StatefulImage::default().resize(resize.to_resize()),
                area,
                buf,
                &mut cover_art.image
            );
        }
    }
}
