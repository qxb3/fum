use ratatui::{layout::Rect, style::Color};
use taffy::{NodeId, TaffyTree};

use crate::FumResult;

/// Which source the slider should be using.
#[derive(Debug, Clone, PartialEq)]
pub enum SliderDataSource {
    Progress,
    Volume,
}

/// Slider filled & remaining text opts.
#[derive(Debug, Clone)]
pub struct SliderTextOpts {
    /// Section's text.
    pub text: String,

    /// Section's fg color.
    pub fg: Color,

    /// Section's bg color.
    pub bg: Color,
}

/// Fum Widget.
#[derive(Debug, Clone)]
pub enum FumWidget {
    /// A container that can manage and contains another widgets.
    Container {
        /// Container children.
        children: Vec<FumWidget>,

        /// The direction of the children will flow.
        direction: taffy::FlexDirection,

        /// The container alignment.
        align: taffy::AlignItems,

        /// The spacing between the children.
        spacing: u16,
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

        /// If the label should be vertical.
        vertical: bool,

        /// Label width.
        width: u16,

        /// Label height.
        height: u16,
    },

    /// For displaying clickable texts.
    Button {
        /// Button text content.
        text: String,

        /// Button function when clicked.
        func: rhai::FnPtr,

        /// If the button should be vertical.
        vertical: bool,

        /// Button width.
        width: u16,

        /// Button height.
        height: u16,
    },

    /// For displaying sliders.
    Slider {
        /// The data source this slider will be using.
        data_source: SliderDataSource,

        /// The filled section of the slider.
        filled: SliderTextOpts,

        /// The remaining un-filled section of the slider.
        remaining: SliderTextOpts,
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
            FumWidget::Container { children, direction, align, spacing, .. } => {
                // Where the node children of this container will be stored.
                let mut children_nodes = Vec::new();

                // Recursively build the tree.
                for child in children {
                    let node = FumWidget::build_taffy_tree(taffy, child)?;
                    children_nodes.push(node);
                }

                // Center the container items if the container direction is horizontal & the align is center.
                let justify_content = if *direction == taffy::FlexDirection::Row && *align == taffy::AlignItems::Center {
                    Some(taffy::JustifyContent::Center)
                } else {
                    None
                };

                // Creates the container parent node.
                let parent_node = taffy.new_with_children(
                    taffy::Style {
                        display: taffy::Display::Flex,
                        flex_direction: *direction,
                        align_items: Some(*align),
                        justify_content,
                        gap: taffy::Size::length(*spacing as f32),
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
                // Creates the label node.
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

            FumWidget::Button { width, height, .. } => {
                let button_node = taffy.new_leaf(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::length(*width as f32),
                        height: taffy::Dimension::length(*height as f32),
                    },
                    ..Default::default()
                })?;

                // Attach widget on this node as a context.
                taffy.set_node_context(button_node, Some(widget.clone()))?;

                Ok(button_node)
            }

            FumWidget::Slider { .. } => {
                let slider_node = taffy.new_leaf(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::length(1.0),
                    },
                    ..Default::default()
                })?;

                // Attach widget on this node as a context.
                taffy.set_node_context(slider_node, Some(widget.clone()))?;

                Ok(slider_node)
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

            FumWidget::Button { .. } => {
                let button_layout = taffy.layout(node)?;

                // Creates a rect out of button layout with adjusted location based on parent rect.
                let button_rect = Rect::new(
                    button_layout.location.x as u16 + parent_rect.x,
                    button_layout.location.y as u16 + parent_rect.y,
                    button_layout.size.width as u16,
                    button_layout.size.height as u16,
                );

                rects.push((button_rect, widget));
            }

            FumWidget::Slider { .. } => {
                let slider_layout = taffy.layout(node)?;

                // Creates a rect out of slider layout with adjusted location based on parent rect.
                let slider_rect = Rect::new(
                    slider_layout.location.x as u16 + parent_rect.x,
                    slider_layout.location.y as u16 + parent_rect.y,
                    slider_layout.size.width as u16,
                    slider_layout.size.height as u16,
                );

                rects.push((slider_rect, widget));
            }
        }

        Ok(())
    }

    // /// Get the size of the widget.
    // pub fn get_size(&self) -> taffy::Size<taffy::Dimension> {
    //     match self {
    //         #[rustfmt::skip]
    //         FumWidget::Container { .. } => taffy::Size::auto(),
    //         #[rustfmt::skip]
    //         FumWidget::CoverImage { width, height, .. } => taffy::Size { width: taffy::Dimension::length(*width as f32), height: taffy::Dimension::length(*height as f32) },
    //         #[rustfmt::skip]
    //         FumWidget::Label { width, height, .. } => taffy::Size { width: taffy::Dimension::length(*width as f32), height: taffy::Dimension::length(*height as f32) },
    //         #[rustfmt::skip]
    //         FumWidget::Button { width, height, .. } => taffy::Size { width: taffy::Dimension::length(*width as f32), height: taffy::Dimension::length(*height as f32) },
    //         #[rustfmt::skip]
    //         FumWidget::Slider { .. } => taffy::Size::auto(),
    //     }
    // }
}
