use ratatui::layout::Rect;
use taffy::prelude::TaffyAuto;

use crate::{
    script::{location::UiLocation, ScriptTaffy, ScriptUi},
    widget::FumWidget,
};

use super::ScriptFnResult;

/// UI() function to set or update the ui.
pub fn ui(
    taffy: ScriptTaffy,
    ui_state: ScriptUi,
) -> impl Fn(UiLocation, rhai::Array) -> ScriptFnResult<()> {
    move |location: UiLocation, widgets: rhai::Array| -> ScriptFnResult<()> {
        // Acquire lock for taffy.
        let mut taffy = taffy
            .lock()
            .map_err(|err| format!("Failed to acquire lock for taffy: {err}"))?;

        // Where the children of the root nodes will be stored.
        let mut widget_nodes = Vec::new();

        // Build taffy tree.
        for widget in widgets {
            let widget = widget
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The values of UI() function needs to be a widget")?;

            // Build the widget node.
            let node = FumWidget::build_taffy_tree(&mut *taffy, &widget)
                .map_err(|err| format!("Failed to create layout tree: {err}"))?;

            widget_nodes.push(node);
        }

        // Creates the root node that will contain the ui layout.
        let root_node = taffy
            .new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    size: taffy::Size {
                        width: taffy::Dimension::AUTO,
                        height: taffy::Dimension::AUTO,
                    },
                    ..Default::default()
                },
                &widget_nodes,
            )
            .map_err(|err| format!("Failed to create root node for the ui: {err}"))?;

        // Fetch the cols & rows of the terminal.
        let (term_cols, term_rows) = crossterm::terminal::size().map_err(|err| {
            format!("Failed to fetch the terminal width & height: {err}")
        })?;

        // Resolve the location on where fum should be located.
        let (window_node_align_items, window_node_justify_content) =
            location.resolve_location();

        // Creates the terminal node for positioning where in the terminal should fum be displayed.
        let window_node = taffy
            .new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    align_items: Some(window_node_align_items),
                    justify_content: Some(window_node_justify_content),
                    min_size: taffy::Size {
                        width: taffy::Dimension::length(term_cols.into()),
                        height: taffy::Dimension::length(term_rows.into()),
                    },
                    ..Default::default()
                },
                &[root_node],
            )
            .map_err(|err| format!("Failed to create foo node for the ui: {err}"))?;

        // Compute the layout
        taffy
            .compute_layout(
                window_node,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(term_cols.into()),
                    height: taffy::AvailableSpace::Definite(term_rows.into()),
                },
            )
            .map_err(|err| format!("Failed to compute the layout: {err}"))?;

        // Layout of the root node.
        let root_layout = taffy
            .layout(root_node)
            .map_err(|err| format!("Failed to get the layout of root node: {err}"))?;

        // Children of the root node.
        let root_children = taffy
            .children(root_node)
            .map_err(|err| format!("Failed to get the children of root node: {err}"))?;

        // Creates the root rect.
        let root_rect = Rect::new(
            root_layout.location.x as u16,
            root_layout.location.y as u16,
            root_layout.size.width as u16,
            root_layout.size.height as u16,
        );

        // Builds rects.
        let mut rects = Vec::new();
        for child in root_children {
            FumWidget::build_rects(&mut rects, &*taffy, &root_rect, child).map_err(
                |err| format!("Failed to build rect on node: {:?} - {err}", child),
            )?;
        }

        // Updates the ui.
        let mut ui = ui_state
            .lock()
            .map_err(|err| format!("Failed to acquire lock for ui state: {err}"))?;

        *ui = rects;

        Ok(())
    }
}
