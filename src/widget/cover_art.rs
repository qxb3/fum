use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::Stylize, text::Text, widgets::{Block, StatefulWidget, Widget}};
use ratatui_image::StatefulImage;

use crate::{get_border, get_color, state::FumState};

use super::FumWidget;

pub fn render(widget: &FumWidget, area: Rect, buf: &mut Buffer, state: &mut FumState) {
    if let FumWidget::CoverArt { resize, border, bg, fg, .. } = widget {
        let (bg, _) = get_color!(bg, fg, &state.parent_bg, &state.parent_fg);

        let border = get_border!(border);

        // Render bg
        Block::new()
            .borders(border)
            .bg(*bg)
            .render(area, buf);

        if let Some(cover_art) = state.meta.cover_art.as_mut() {
            StatefulWidget::render(
                StatefulImage::default().resize(resize.to_resize()),
                area,
                buf,
                &mut cover_art.image
            );
        } else {
            let split = state.cover_art_ascii.split('\n');
            let width = split.to_owned().map(|line| line.len() as u16).max().unwrap_or(0);
            let height = split.count() as u16;

            let [ascii_area] = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center).areas(area);
            let [ascii_area] = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center).areas(ascii_area);

            Text::from(state.cover_art_ascii.as_str())
                .render(ascii_area, buf);
        }
    }
}
