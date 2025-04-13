use ratatui::layout::Rect;
use rhai::EvalAltResult;
use taffy::prelude::TaffyAuto;

use crate::widget::FumWidget;

use super::{ScriptTaffy, ScriptUi};

/// Type alias for Result at script function calls.
type FnResult<T> = Result<T, Box<EvalAltResult>>;

/// FUM_UI() function to set or update the ui.
pub fn fum_ui(taffy: ScriptTaffy, ui: ScriptUi) -> impl Fn(rhai::Array) -> FnResult<()> {
    move |widgets: rhai::Array| -> FnResult<()> {
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
                .map_err(|_| "The values of FUM_UI() function needs to be a widget")?;

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

        // Creates the terminal node for positioning where in the terminal should fum be displayed.
        let window_node = taffy
            .new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    align_items: Some(taffy::AlignItems::Center),
                    justify_content: Some(taffy::JustifyContent::Center),
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

/// Container() widget function.
pub fn container() -> impl Fn(taffy::FlexDirection, rhai::Array) -> FnResult<FumWidget> {
    move |direction: taffy::FlexDirection, children: rhai::Array| -> FnResult<FumWidget> {
        // Where the nodes of container children will be stored.
        let mut container_children = Vec::new();

        // width & height of container.
        let mut container_width = 0;
        let mut container_height = 0;

        for child in children {
            let child = child
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The children of the container needs to be a widget")?;

            // Adds the child size to the container size.
            container_width += child.get_width();
            container_height += child.get_height();

            container_children.push(child);
        }

        Ok(FumWidget::Container {
            children: container_children,
            direction,
            align: taffy::AlignItems::Start,
            width: container_width,
            height: container_height,
        })
    }
}

/// Container() widget function with opts.
pub fn container_opts() -> impl Fn(rhai::Map) -> FnResult<FumWidget> {
    move |opts: rhai::Map| -> FnResult<FumWidget> {
        // Extract container direction from opts.
        let direction = opts
            .get("direction")
            .cloned()
            .ok_or("Container widget needs to have a direction")?
            .try_cast_result::<taffy::FlexDirection>()
            .map_err(|_| "Container direction needs to be a valid direction")?;

        // Extract container alignment from opts, Will default to START if it doesnt exists.
        let align = opts
            .get("align")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(taffy::AlignItems::Start))
            .try_cast_result::<taffy::AlignItems>()
            .map_err(|_| "Container align needs to be a valid align value")?;

        // Extract container children from opts.
        let children = opts
            .get("children")
            .cloned()
            .ok_or("Container widget needs to have children")?
            .try_cast_result::<rhai::Array>()
            .map_err(|_| "Container children needs to be an array of widget")?;

        // Where the nodes of container children will be stored.
        let mut container_children = Vec::new();

        // width & height of container.
        let mut container_width = 0;
        let mut container_height = 0;

        for child in children {
            let child = child
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The children of the container needs to be a widget")?;

            // Adds the child size to the container size.
            container_width += child.get_width();
            container_height += child.get_height();

            container_children.push(child);
        }

        Ok(FumWidget::Container {
            children: container_children,
            direction,
            align,
            width: container_width,
            height: container_height,
        })
    }
}

/// CoverImage() widget function.
pub fn cover_image() -> impl Fn(rhai::INT, rhai::INT) -> FnResult<FumWidget> {
    move |width: rhai::INT, height: rhai::INT| -> FnResult<FumWidget> {
        Ok(FumWidget::CoverImage {
            width: width as u16,
            height: height as u16,
        })
    }
}

/// Label() widget function.
pub fn label() -> impl Fn(rhai::Dynamic) -> FnResult<FumWidget> {
    move |text: rhai::Dynamic| -> FnResult<FumWidget> {
        let text = text.to_string();

        Ok(FumWidget::Label {
            text: text.to_string(),
            width: text.len() as u16,
            height: 1,
        })
    }
}
