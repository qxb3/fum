use std::{error::Error, sync::Arc};

use fum::Fum;
use ratatui::layout::Rect;
use script::Script;
use taffy::prelude::TaffyMinContent;
use widget::FumWidget;

mod cli;
mod event;
mod fum;
mod mode;
mod mpris;
mod script;
mod state;
mod track;
mod ui;
mod widget;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    // None variant signifies that we shouldn't start fum tui.
    if let Some(args) = cli::run().await? {
        // let mut script = Script::from_file(&args.config_path)?;
        // script.execute()?;
        //
        // let taffy_arc = Arc::clone(&script.taffy);
        // let mut taffy = taffy_arc
        //     .lock()
        //     .map_err(|err| format!("Failed to acquire lock for taffy: {err}"))?;
        //
        // let ui_arc = Arc::clone(&script.ui);
        // let ui = ui_arc
        //     .lock()
        //     .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;
        //
        // // Only draw if we even have a ui.
        // if let Some(ui) = &*ui {
        //     // Compute the layout
        //     taffy.compute_layout(*ui, taffy::Size::MIN_CONTENT)?;
        //
        //     let root_layout = taffy.layout(*ui)?;
        //     // let root_rect = Rect::new(
        //     //     root_layout.location.x as u16,
        //     //     root_layout.location.y as u16,
        //     //     root_layout.size.width as u16,
        //     //     root_layout.size.height as u16,
        //     // );
        //
        //     let root_children = taffy.children(*ui)?;
        //     let mut children_rects = Vec::new();
        //
        //     for child in root_children {
        //         FumWidget::build_rects(
        //             &mut children_rects,
        //             &*taffy,
        //             root_layout,
        //             child
        //         )?;
        //     }
        //
        //     // println!("ROOT = {:#?}", root_rect);
        //     println!("CHILDREN = {:#?}", children_rects);
        // }


        let mut fum = Fum::new(&args.config_path).await?;
        fum.start(args.mode).await?;
    }

    Ok(())
}
