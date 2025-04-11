use std::sync::Arc;

use ratatui::{layout::Rect, prelude::CrosstermBackend, style::{Color, Stylize}, text::Text, widgets::Block, Terminal};
use ratatui_image::StatefulImage;

use crate::{state::State, widget::FumWidget, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut State,
    ui: &Vec<(Rect, FumWidget)>,
) -> FumResult<()> {
    // Gets the current cover state.
    let current_cover_arc = Arc::clone(&state.current_cover);
    let mut current_cover = current_cover_arc.lock().await;

    // Drawing part.
    terminal.draw(|frame| {
        for (rect, widget) in ui {
            match widget {
                FumWidget::CoverImage { .. } => {
                    if let Some(cover) = current_cover.as_mut() {
                        frame.render_widget(Block::new().bg(Color::Red), *rect);
                        frame.render_stateful_widget(
                            StatefulImage::default(),
                            *rect,
                            cover,
                        );
                    }
                }

                FumWidget::Label { text, .. } => {
                    frame.render_widget(Text::from(text.as_str()), *rect);
                }

                // Ignores the container widget since it will not be on the ui state.
                FumWidget::Container { .. } => unreachable!(),
            }
        }
    })?;

    Ok(())
}
