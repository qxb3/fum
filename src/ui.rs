use ratatui::{layout::Rect, prelude::CrosstermBackend, text::Text, Terminal};
use ratatui_image::StatefulImage;

use crate::{state::State, widget::FumWidget, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut State,
    ui: &Vec<(Rect, FumWidget)>,
) -> FumResult<()> {
    // Gets the current cover state.
    let mut current_cover = state.current_cover.lock().await;

    // Drawing part.
    terminal.draw(|frame| {
        for (rect, widget) in ui {
            match widget {
                FumWidget::CoverImage { .. } => {
                    if let Some(cover) = current_cover.as_mut() {
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

                FumWidget::Button { text, .. } => {
                    frame.render_widget(Text::from(text.as_str()), *rect);
                }

                _ => unreachable!(),
            }
        }
    })?;

    Ok(())
}
