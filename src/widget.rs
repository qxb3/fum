use anyhow::{anyhow, Context};
use ratatui::{layout::Rect, style::Color};
use taffy::{
    AlignItems, Dimension, Display, FlexDirection, JustifyContent, NodeId, Size, Style, TaffyTree,
};

use crate::FumResult;

/// Which source the slider should be using.
#[derive(Debug, Clone)]
pub enum SliderDataSource {
    Progress,
    Volume,
}

/// Slider filled & remaining text opts.
#[derive(Debug, Clone)]
pub struct SliderTextOpts {
    /// Text content.
    pub text: String,

    /// Foreground color.
    pub fg: Color,

    /// Background color.
    pub bg: Color,
}

/// List of fum widgets.
#[derive(Debug, Clone)]
pub enum FumWidgetKind {
    Container {
        /// The childrens of the container.
        children: Vec<FumWidgetKind>,

        /// Which direction the children will flow.
        direction: FlexDirection,

        /// Where should the children be aligned.
        align: AlignItems,

        /// How should the container flex justify behave.
        justify: Option<JustifyContent>,

        /// The spacing between the children.
        spacing: u16,
    },

    CoverImage {
        /// Cover image width.
        width: u16,

        /// Cover image height.
        height: u16,
    },

    Label {
        /// The text content of the label.
        text: String,

        /// Whether the label should be vertical.
        vertical: bool,

        /// The max chars the label will render.
        max_chars: u16,

        /// Foreground color of the label.
        fg: Color,

        /// Background color of the label.
        bg: Color,

        /// Label width.
        width: u16,

        /// Label height.
        height: u16,
    },

    Button {
        /// The widget of this button.
        widget: Box<FumWidgetKind>,

        /// The button function when clicked.
        func: rhai::FnPtr,
    },

    Slider {
        /// The data source this slider will be using.
        data_source: SliderDataSource,

        /// The filled section of the slider.
        filled: SliderTextOpts,

        /// The remaining un-filled section of the slider.
        remaining: SliderTextOpts,
    },
}

/// A wrapper around FumWidgetKind.
#[derive(Debug, Clone)]
pub struct FumWidget {
    /// The rect of the widget.
    rect: Rect,

    /// The kind of the widget along with its data.
    kind: FumWidgetKind,
}

unsafe impl Send for FumWidget {}
unsafe impl Sync for FumWidget {}

impl FumWidgetKind {
    /// Creates a taffy node.
    pub fn create_node(&self, taffy: &mut TaffyTree<Self>) -> FumResult<NodeId> {
        let node = match self {
            Self::Container {
                children,
                direction,
                align,
                justify,
                spacing,
            } => {
                // Recursively creates nodes for the container children.
                let mut nodes = vec![];
                for child in children {
                    let node = child.create_node(taffy)?;
                    nodes.push(node);
                }

                // If justify from opts is set, Use that else:
                // Center the container items if the container direction is horizontal & the align is center.
                let justify_content = match justify {
                    Some(justify) => Some(*justify),
                    None => match (direction, align) {
                        (FlexDirection::Row, AlignItems::Center) => Some(JustifyContent::Center),
                        _ => None,
                    },
                };

                // Creates the container node.
                let container_node = taffy
                    .new_with_children(
                        Style {
                            display: Display::Flex,
                            flex_direction: *direction,
                            align_items: Some(*align),
                            justify_content,
                            gap: Size::length(*spacing as f32),
                            size: Size {
                                width: Dimension::percent(1.0), // Fill the available space horizontally.
                                height: Dimension::percent(1.0), // Fill the available space vertically.
                            },
                            ..Default::default()
                        },
                        &nodes,
                    )
                    .context("Failed to create container node")?;

                container_node
            }

            Self::CoverImage { width, height } => {
                // Absolute size for cover image.
                let size = Size {
                    width: Dimension::length(*width as f32),
                    height: Dimension::length(*height as f32),
                };

                // Creates the cover image node.
                let cover_img_node = taffy
                    .new_leaf(Style {
                        size,
                        min_size: size,
                        max_size: size,
                        ..Default::default()
                    })
                    .context("Failed to create cover image node")?;

                cover_img_node
            }

            Self::Label { width, height, .. } => {
                // Creates the label node.
                let label_node = taffy
                    .new_leaf(Style {
                        size: Size {
                            width: Dimension::length(*width as f32),
                            height: Dimension::length(*height as f32),
                        },
                        ..Default::default()
                    })
                    .context("Failed to create label node")?;

                label_node
            }

            Self::Button { widget, .. } => {
                // Recursively create the node of the button widget.
                let widget_node = widget.create_node(taffy)?;

                // Creates the button node.
                let button_node = taffy.new_with_children(Style::default(), &[widget_node])?;

                button_node
            }

            Self::Slider { .. } => {
                // Absolute size for slider widget.
                let size = Size {
                    width: Dimension::percent(1.0),
                    height: Dimension::length(1.0),
                };

                // Creates the slider node.
                let slider_node = taffy
                    .new_leaf(Style {
                        size,
                        min_size: size,
                        max_size: size,
                        ..Default::default()
                    })
                    .context("Failed to create slider node")?;

                slider_node
            }
        };

        // Sets the node context to the widget.
        // As much as i would like to not clone widget here any other else is gonna over complicate things (im lazy dont ask me about it).
        taffy
            .set_node_context(node, Some(self.clone()))
            .context("Failed to set the widget node context")?;

        Ok(node)
    }
}

impl FumWidget {
    pub fn new(rect: Rect, kind: FumWidgetKind) -> Self {
        Self { rect, kind }
    }

    /// Creates flat ratatui rects based on taffy nodes.
    pub fn create_layout(
        taffy: &TaffyTree<FumWidgetKind>,
        node: NodeId,
        parent_rect: &Rect,
        layout: &mut Vec<FumWidget>,
    ) -> FumResult<()> {
        let widget_kind = taffy
            .get_node_context(node)
            .ok_or(anyhow!("Expected a context on a node but got nothing"))?
            .clone();

        match widget_kind {
            FumWidgetKind::Container { .. } => {
                // Gets the node layout.
                let container_layout = taffy
                    .layout(node)
                    .context("Failed to get layout on container node")?;

                // Creates a rect based on the node layout.
                let container_rect = Rect::new(
                    container_layout.location.x as u16 + parent_rect.x,
                    container_layout.location.y as u16 + parent_rect.y,
                    container_layout.size.width as u16,
                    container_layout.size.height as u16,
                );

                // Gets the node children.
                let container_children = taffy
                    .children(node)
                    .context("Failed to get children on container node")?;

                // Recursively keep adding layout widgets.
                for child in container_children {
                    FumWidget::create_layout(taffy, child, &container_rect, layout)?;
                }
            }

            FumWidgetKind::CoverImage { .. }
            | FumWidgetKind::Label { .. }
            | FumWidgetKind::Button { .. }
            | FumWidgetKind::Slider { .. } => {
                // Gets the node layout.
                let widget_layout = taffy.layout(node).context("Failed to get layout on node")?;

                // Creates a rect based on the node layout.
                let widget_rect = Rect::new(
                    widget_layout.location.x as u16 + parent_rect.x,
                    widget_layout.location.y as u16 + parent_rect.y,
                    widget_layout.size.width as u16,
                    widget_layout.size.height as u16,
                );

                layout.push(FumWidget::new(widget_rect, widget_kind));
            }
        }

        Ok(())
    }
}
