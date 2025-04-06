use ratatui::{layout::Rect, prelude::CrosstermBackend, text::Text, Terminal};

use crate::{state::State, widget::FumWidget, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    _state: &mut State,
    ui: &Vec<(Rect, FumWidget)>,
) -> FumResult<()> {
    // Drawing part.
    terminal.draw(|frame| {
        for (rect, widget) in ui {
            match widget {
                FumWidget::Label { text, .. } => {
                    frame.render_widget(Text::from(text.as_str()), *rect);
                }

                _ => {}
            }
        }
    })?;

    Ok(())
}
