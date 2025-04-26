use std::sync::Arc;

use ratatui::layout::Rect;
use taffy::prelude::TaffyAuto;

use crate::{
    script::{location::UiLocation, ScriptTaffy, ScriptUi},
    widget::FumWidget,
};

use super::ScriptFnResult;

/// UI() function to set or update the ui with opts.
pub fn ui_opts(
    taffy: ScriptTaffy,
    ui: ScriptUi,
) -> impl Fn(rhai::Map) -> ScriptFnResult<()> {
    move |opts: rhai::Map| -> ScriptFnResult<()> {
        // Extract location from opts.
        let location = opts
            .get("location")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(UiLocation::Center))
            .try_cast_result::<UiLocation>()
            .map_err(|_| "UI `location` needs to be a valid location")?;

        // Extract layout from opts.
        let layout = opts
            .get("layout")
            .cloned()
            .ok_or("UI needs to have a `layout`")?
            .try_cast_result::<rhai::Array>()
            .map_err(|_| "UI `layout` needs to be a array of widgets")?;

        // Extract min_width from opts.
        let min_width = opts
            .get("min_width")
            .cloned()
            .and_then(|s| s.try_cast::<rhai::INT>());

        // Extract min_height from opts.
        let min_height = opts
            .get("min_height")
            .cloned()
            .and_then(|s| s.try_cast::<rhai::INT>());

        // Acquire lock for taffy.
        let mut taffy = taffy
            .lock()
            .map_err(|err| format!("Failed to acquire lock for taffy: {err}"))?;

        // Where the children of the root nodes will be stored.
        let mut widget_nodes = Vec::new();

        // Build taffy tree.
        for widget in layout {
            let widget = widget
                .try_cast_result::<FumWidget>()
                .map_err(|_| "UI `layout` needs to be a array of widgets")?;

            // Build the widget node.
            let node = FumWidget::build_taffy_tree(&mut *taffy, &widget)
                .map_err(|err| format!("Failed to create layout tree: {err}"))?;

            widget_nodes.push(node);
        }

        // Gets the root node size based on the min width & height.
        let root_node_size = match (min_width, min_height) {
            (Some(min_width), None) => taffy::Size {
                width: taffy::Dimension::length(min_width as f32),
                height: taffy::Dimension::auto(),
            },
            (None, Some(min_height)) => taffy::Size {
                width: taffy::Dimension::auto(),
                height: taffy::Dimension::length(min_height as f32),
            },
            (Some(min_width), Some(min_height)) => taffy::Size {
                width: taffy::Dimension::length(min_width as f32),
                height: taffy::Dimension::length(min_height as f32),
            },
            (None, None) => taffy::Size {
                width: taffy::Dimension::auto(),
                height: taffy::Dimension::auto(),
            },
        };

        // Creates the root node that will contain the ui layout.
        let root_node = taffy
            .new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    size: root_node_size,
                    min_size: root_node_size,
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
        let mut ui = ui
            .lock()
            .map_err(|err| format!("Failed to acquire lock for ui state: {err}"))?;

        *ui = rects;

        Ok(())
    }
}

/// Wrapper arround the ui_opts.
pub fn ui(
    taffy: ScriptTaffy,
    ui: ScriptUi,
) -> impl Fn(UiLocation, rhai::Array) -> ScriptFnResult<()> {
    move |location: UiLocation, layout: rhai::Array| -> ScriptFnResult<()> {
        let mut opts = rhai::Map::new();
        opts.insert("location".into(), rhai::Dynamic::from(location));
        opts.insert("layout".into(), rhai::Dynamic::from_array(layout));

        let ui_opts = ui_opts(Arc::clone(&taffy), Arc::clone(&ui));
        ui_opts(opts)
    }
}

/// Wrapper arround the ui_opts & can pass extra opts.
pub fn ui_ext_opts(
    taffy: ScriptTaffy,
    ui: ScriptUi,
) -> impl Fn(UiLocation, rhai::Array, rhai::Map) -> ScriptFnResult<()> {
    move |location: UiLocation,
          layout: rhai::Array,
          ext_opts: rhai::Map|
          -> ScriptFnResult<()> {
        let mut opts = rhai::Map::new();
        opts.insert("location".into(), rhai::Dynamic::from(location));
        opts.insert("layout".into(), rhai::Dynamic::from_array(layout));
        opts.extend(ext_opts);

        let ui_opts = ui_opts(Arc::clone(&taffy), Arc::clone(&ui));
        ui_opts(opts)
    }
}
