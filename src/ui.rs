use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::Stylize,
    text::Text,
    widgets::{Paragraph, Wrap},
    Terminal,
};
use ratatui_image::StatefulImage;

use crate::{state::State, utils::text::truncate, widget::FumWidget, FumResult};

pub async fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut State,
    ui: &Vec<(Rect, FumWidget)>,
) -> FumResult<()> {
    // Gets the current track state.
    let current_track = state.current_track.lock().await;

    // Gets the current cover state.
    let mut current_cover = state.current_cover.lock().await;

    // Drawing part.
    terminal.draw(|frame| {
        for (rect, widget) in ui {
            match widget {
                // Render CoverImage Widget.
                FumWidget::CoverImage { .. } => {
                    if let Some(cover) = current_cover.as_mut() {
                        frame.render_stateful_widget(
                            StatefulImage::default(),
                            *rect,
                            &mut cover.stateful_protocol,
                        );
                    }
                }

                // Render Label Widget.
                FumWidget::Label {
                    text,
                    max_chars,
                    fg,
                    bg,
                    ..
                } => {
                    // Truncate the text if the max_chars isnt a -1.
                    let text = if *max_chars == -1 {
                        text.to_string()
                    } else {
                        truncate(&text, *max_chars as usize)
                    };

                    frame.render_widget(
                        Paragraph::new(text).wrap(Wrap::default()).fg(*fg).bg(*bg),
                        *rect,
                    );
                }

                // Render Button Widget.
                FumWidget::Button { text, fg, bg, .. } => {
                    frame.render_widget(
                        Paragraph::new(text.to_string())
                            .wrap(Wrap::default())
                            .fg(*fg)
                            .bg(*bg),
                        *rect,
                    );
                }

                // Render Slider Widget.
                FumWidget::Slider {
                    filled, remaining, ..
                } => {
                    // Calculates the ratio for filled and remaining.
                    let filled_remaining_ratio = if current_track.position.as_secs() > 0 {
                        current_track.position.as_secs() as f64
                            / current_track.length.as_secs() as f64
                    } else {
                        0.0
                    };

                    // Calculates the width for both filled & remaining space.
                    let filled_width =
                        (filled_remaining_ratio * rect.width as f64).round();
                    let remaining_width = rect.width.saturating_sub(filled_width as u16);

                    // Creates a layout rects for filled & remaining area.
                    let [filled_area, remaining_area] = Layout::horizontal([
                        Constraint::Length(filled_width as u16),
                        Constraint::Length(remaining_width as u16),
                    ])
                    .areas(*rect);

                    // Gets the final string of filled & remaining to be rendered.
                    let filled_result = filled.text.repeat(filled_width as usize);
                    let remaining_result =
                        remaining.text.repeat(remaining_width as usize);

                    // Render filled.
                    frame.render_widget(
                        Text::from(filled_result.as_str())
                            .fg(filled.fg)
                            .bg(filled.bg),
                        filled_area,
                    );

                    // Render remaining.
                    frame.render_widget(
                        Text::from(remaining_result.as_str())
                            .fg(remaining.fg)
                            .bg(remaining.bg),
                        remaining_area,
                    );
                }

                _ => unreachable!(),
            }
        }
    })?;

    Ok(())
}
