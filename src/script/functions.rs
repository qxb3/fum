use ratatui::layout::Rect;
use rhai::EvalAltResult;
use taffy::prelude::TaffyMinContent;

use crate::widget::FumWidget;

use super::{ScriptTaffy, ScriptUi};

/// Type alias for Result at script function calls.
type FnResult<T> = Result<T, Box<EvalAltResult>>;

/// FUM_UI() function to set or update the ui.
pub fn fum_ui(taffy: ScriptTaffy, ui: ScriptUi) -> impl Fn(rhai::Array) -> FnResult<()> {
    move |widgets: rhai::Array| -> FnResult<()> {
        let mut taffy = taffy
            .lock()
            .map_err(|err| format!("Failed to acquire lock for taffy: {err}"))?;

        // Where the children of the root nodes will be stored.
        let mut widget_nodes = Vec::new();

        for widget in widgets {
            let widget = widget
                .try_cast_result::<FumWidget>()
                .map_err(|_| "The values of FUM_UI() function needs to be a widget")?;

            // Build the widget node.
            let node = FumWidget::build_taffy_tree(&mut *taffy, &widget)
                .map_err(|err| format!("Failed to create layout tree: {err}"))?;

            widget_nodes.push(node);
        }

        // Creates the very root node of the ui.
        let root_node = taffy
            .new_with_children(
                taffy::Style {
                    display: taffy::Display::Flex,
                    flex_direction: taffy::FlexDirection::Column,
                    ..Default::default()
                },
                &widget_nodes,
            )
            .map_err(|err| format!("Failed to create root node for the ui: {err}"))?;

        // Compute the layout
        taffy
            .compute_layout(root_node, taffy::Size::MIN_CONTENT)
            .map_err(|err| format!("Failed to compute the layout: {err}"))?;

        let root_layout = taffy
            .layout(root_node)
            .map_err(|err| format!("Failed to get the layout of root node: {err}"))?;

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
