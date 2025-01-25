use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use ratatui_image::StatefulImage;

use super::{FumWidget, FumWidgetState};

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumWidgetState) {
    if let FumWidget::CoverArt { .. } = widget {
        if let Some(cover_art) = state.meta.cover_art.as_mut() {
            StatefulWidget::render(
                StatefulImage::default(),
                area,
                buf,
                &mut cover_art.image
            );
        }
    }
}
