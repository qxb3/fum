use ratatui::layout::Rect;
use taffy::{NodeId, TaffyTree};

use crate::FumResult;

/// Fum Widget.
#[derive(Debug, PartialEq, Clone)]
pub enum FumWidget {
    /// A container that can manage and contains another widgets.
    Container {
        /// Container children.
        children: Vec<FumWidget>,

        /// The direction of the children will flow.
        direction: taffy::FlexDirection,

        /// The container alignment.
        align: taffy::AlignItems,

        /// Container total width.
        width: u16,

        /// Container total height.
        height: u16,
    },

    /// For displaying the cover art image of a track.
    CoverImage {
        /// Cover image width.
        width: u16,

        /// Cover image height.
        height: u16,
    },

    /// For displaying texts.
    Label {
        /// Label text content.
        text: String,

        /// Label width.
        width: u16,

        /// Label height.
        height: u16,
    },
}

impl FumWidget {
    /// Builds taffy tree.
    pub fn build_taffy_tree(
        taffy: &mut TaffyTree<FumWidget>,
        widget: &FumWidget,
    ) -> FumResult<NodeId> {
        match widget {
            #[rustfmt::skip]
            FumWidget::Container { children, direction, align, .. } => {
                // Where the node children of this container will be stored.
                let mut children_nodes = Vec::new();

                // Recursively build the tree.
                for child in children {
                    let node = FumWidget::build_taffy_tree(taffy, child)?;
                    children_nodes.push(node);
                }

                // Creates the container parent node.
                let parent_node = taffy.new_with_children(
                    taffy::Style {
                        display: taffy::Display::Flex,
                        flex_direction: *direction,
                        align_items: Some(*align),
                        ..Default::default()
                    },
                    &children_nodes,
                )?;

                // Attach widget on this node as a context.
                taffy.set_node_context(parent_node, Some(widget.clone()))?;

                Ok(parent_node)
            }

            FumWidget::CoverImage { width, height } => {
                // Creates the cover image node.
                let cover_img_node = taffy.new_leaf(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(*width as f32),
                        height: taffy::Dimension::length(*height as f32),
                    },
                    ..Default::default()
                })?;

                // Attach widget on this node as a context.
                taffy.set_node_context(cover_img_node, Some(widget.clone()))?;

                Ok(cover_img_node)
            }

            FumWidget::Label { width, height, .. } => {
                // Craetes the label node.
                let label_node = taffy.new_leaf(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(*width as f32),
                        height: taffy::Dimension::length(*height as f32),
                    },
                    ..Default::default()
                })?;

                // Attach widget on this node as a context.
                taffy.set_node_context(label_node, Some(widget.clone()))?;

                Ok(label_node)
            }
        }
    }

    /// Creates a Vec of ratatui rects out of taffy nodes.
    pub fn build_rects<'a>(
        rects: &mut Vec<(Rect, FumWidget)>,
        taffy: &'a TaffyTree<FumWidget>,
        parent_rect: &Rect,
        node: NodeId,
    ) -> FumResult<()> {
        // Gets out the widget context node.
        let widget = taffy
            .get_node_context(node)
            .ok_or("Expected a widget context on a node but got None")?
            .clone();

        match widget {
            FumWidget::Container { .. } => {
                let container_layout = taffy.layout(node)?;
                let container_children = taffy.children(node)?;

                // Creates a rect out of container layout with adjusted location based on parent rect.
                let container_rect = Rect::new(
                    container_layout.location.x as u16 + parent_rect.x,
                    container_layout.location.y as u16 + parent_rect.y,
                    container_layout.size.width as u16,
                    container_layout.size.height as u16,
                );

                // Recursively builds rects.
                for child in container_children {
                    FumWidget::build_rects(rects, taffy, &container_rect, child)?;
                }
            }

            FumWidget::CoverImage { .. } => {
                let cover_img_layout = taffy.layout(node)?;

                // Creates a rect out of cover image layout with adjusted location based on parent rect.
                let cover_img_rect = Rect::new(
                    cover_img_layout.location.x as u16 + parent_rect.x,
                    cover_img_layout.location.y as u16 + parent_rect.y,
                    cover_img_layout.size.width as u16,
                    cover_img_layout.size.height as u16,
                );

                rects.push((cover_img_rect, widget));
            }

            FumWidget::Label { .. } => {
                let label_layout = taffy.layout(node)?;

                // Creates a rect out of label layout with adjusted location based on parent rect.
                let label_rect = Rect::new(
                    label_layout.location.x as u16 + parent_rect.x,
                    label_layout.location.y as u16 + parent_rect.y,
                    label_layout.size.width as u16,
                    label_layout.size.height as u16,
                );

                rects.push((label_rect, widget));
            }
        }

        Ok(())
    }

    /// Get the width of the widget.
    pub fn get_width(&self) -> u16 {
        match self {
            #[rustfmt::skip]
            FumWidget::Container { width, .. } => *width,
            #[rustfmt::skip]
            FumWidget::CoverImage { width, .. } => *width,
            #[rustfmt::skip]
            FumWidget::Label { width, .. } => *width,
        }
    }

    /// Get the height of the widget.
    pub fn get_height(&self) -> u16 {
        match self {
            #[rustfmt::skip]
            FumWidget::Container { height, .. } => *height,
            #[rustfmt::skip]
            FumWidget::CoverImage { height, .. } => *height,
            #[rustfmt::skip]
            FumWidget::Label { height, .. } => *height,
        }
    }
}
