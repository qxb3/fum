use anyhow::anyhow;
use ratatui::layout::Rect;
use taffy::{
    AlignItems, AvailableSpace, Dimension, Display, FlexDirection, JustifyContent, Size, Style,
    TaffyTree,
};

use crate::{
    event::{Event, EventSender, ScriptEvent},
    widget::{FumWidget, FumWidgetKind},
};

/// Where in the terminal should the layout be positioned.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutLocation {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Center,
}

impl LayoutLocation {
    /// Resolves to taffy's AlignItems & JustifyContent.
    pub fn resolve(&self) -> (AlignItems, JustifyContent) {
        match self {
            LayoutLocation::Top => (AlignItems::Start, JustifyContent::Center),
            LayoutLocation::TopLeft => (AlignItems::Start, JustifyContent::Start),
            LayoutLocation::TopRight => (AlignItems::Start, JustifyContent::End),

            LayoutLocation::Bottom => (AlignItems::End, JustifyContent::Center),
            LayoutLocation::BottomLeft => (AlignItems::End, JustifyContent::Start),
            LayoutLocation::BottomRight => (AlignItems::End, JustifyContent::End),

            LayoutLocation::Left => (AlignItems::Center, JustifyContent::Start),
            LayoutLocation::Right => (AlignItems::Center, JustifyContent::End),
            LayoutLocation::Center => (AlignItems::Center, JustifyContent::Center),
        }
    }
}

/// raw function of LAYOUT().
pub fn layout_function_raw(event_sender: EventSender) -> impl Fn(rhai::Map) {
    move |opts: rhai::Map| {
        // Extract location from opts.
        let location = match opts
            .get("location")
            .cloned()
            .unwrap_or(rhai::Dynamic::from(LayoutLocation::Center))
            .try_cast_result::<LayoutLocation>()
        {
            Ok(location) => location,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("UI `location` needs to be a valid location")))
                    .unwrap();

                return;
            }
        };

        // Extract layout from opts.
        let layout_opt = match opts.get("layout").cloned() {
            Some(layout_opt) => layout_opt,
            None => {
                event_sender
                    .send(Err(anyhow!("UI needs to have a `layout`")))
                    .unwrap();

                return;
            }
        };

        // Parse the layout opt.
        let layout = match layout_opt.try_cast_result::<rhai::Array>() {
            Ok(location) => location,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("UI `layout` needs to be a array of widgets")))
                    .unwrap();

                return;
            }
        };

        // Extract min_width from opts.
        let min_width = match opts.get("min_width").cloned() {
            Some(min_width) => match min_width.try_cast_result::<rhai::INT>() {
                Ok(min_width) => Some(min_width),
                Err(_) => {
                    event_sender
                        .send(Err(anyhow!("UI `min_width` needs to be a valid number")))
                        .unwrap();

                    return;
                }
            },
            None => None,
        };

        // Extract min_height from opts.
        let min_height = match opts.get("min_height").cloned() {
            Some(min_height) => match min_height.try_cast_result::<rhai::INT>() {
                Ok(min_height) => Some(min_height),
                Err(_) => {
                    event_sender
                        .send(Err(anyhow!("UI `min_height` needs to be a valid number")))
                        .unwrap();

                    return;
                }
            },
            None => None,
        };

        // Gets the root node size based on the min width & height.
        let root_node_size = match (min_width, min_height) {
            (Some(min_width), None) => Size {
                width: Dimension::length(min_width as f32),
                height: Dimension::auto(),
            },
            (None, Some(min_height)) => Size {
                width: Dimension::auto(),
                height: Dimension::length(min_height as f32),
            },
            (Some(min_width), Some(min_height)) => Size {
                width: Dimension::length(min_width as f32),
                height: Dimension::length(min_height as f32),
            },
            (None, None) => Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
        };

        // Creates a new taffy engine instance.
        let mut taffy: TaffyTree<FumWidgetKind> = TaffyTree::new();

        let mut nodes = vec![];
        for widget in layout {
            // Parse the widget from layout opt.
            let widget_res = match widget.try_cast_result::<Option<FumWidgetKind>>() {
                Ok(widget) => widget,
                Err(_) => {
                    event_sender
                        .send(Err(anyhow!("UI `layout` needs to be a array of widgets")))
                        .unwrap();

                    return;
                }
            };

            // A None widget means it errors we skip those.
            if let Some(widget) = widget_res {
                // Creates a node.
                let node = match widget.create_node(&mut taffy) {
                    Ok(node) => node,
                    Err(err) => {
                        event_sender.send(Err(err)).unwrap();

                        return;
                    }
                };

                nodes.push(node);
            }
        }

        // Creates the root node that will contain the widgets.
        let root_node = match taffy.new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: root_node_size,
                min_size: root_node_size,
                ..Default::default()
            },
            &nodes,
        ) {
            Ok(root_node) => root_node,
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Failed to create the root node: {err}")))
                    .unwrap();

                return;
            }
        };

        // Fetch the cols & rows of the terminal.
        let (term_cols, term_rows) = match crossterm::terminal::size() {
            Ok((cols, rows)) => (cols, rows),
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Failed to fetch the terminal width & height: {err}")))
                    .unwrap();

                return;
            }
        };

        // Resolve the location on where the layout should be positioned.
        let (window_node_align_items, window_node_justify_content) = location.resolve();

        // Creates the terminal node for positioning where in the terminal should fum be displayed.
        let terminal_node = match taffy.new_with_children(
            Style {
                display: Display::Flex,
                align_items: Some(window_node_align_items),
                justify_content: Some(window_node_justify_content),
                min_size: Size {
                    width: Dimension::length(term_cols.into()),
                    height: Dimension::length(term_rows.into()),
                },
                ..Default::default()
            },
            &[root_node],
        ) {
            Ok(window_node) => window_node,
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Failed to create the window node: {err}")))
                    .unwrap();

                return;
            }
        };

        // Computes the layout.
        if let Err(err) = taffy.compute_layout(
            terminal_node,
            Size {
                width: AvailableSpace::Definite(term_cols.into()),
                height: AvailableSpace::Definite(term_rows.into()),
            },
        ) {
            event_sender
                .send(Err(anyhow!("Failed to compute the layout: {err}")))
                .unwrap();

            return;
        }

        // Layout of the root node.
        let root_layout = match taffy.layout(root_node) {
            Ok(root_node) => root_node,
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Failed to get the layout of root node: {err}")))
                    .unwrap();

                return;
            }
        };

        // Children of the root node.
        let root_children = match taffy.children(root_node) {
            Ok(root_children) => root_children,
            Err(err) => {
                event_sender
                    .send(Err(anyhow!("Failed to get the children of root node: {err}")))
                    .unwrap();

                return;
            }
        };

        // Creates the root rect.
        let root_rect = Rect::new(
            root_layout.location.x as u16,
            root_layout.location.y as u16,
            root_layout.size.width as u16,
            root_layout.size.height as u16,
        );

        // Create the layout.
        let mut layout = vec![];
        for child in root_children {
            if let Err(err) = FumWidget::create_layout(&taffy, child, &root_rect, &mut layout) {
                event_sender.send(Err(err)).unwrap();

                return;
            }
        }

        // Sends out LayoutUpdated event.
        event_sender
            .send(Ok(Event::Script(ScriptEvent::LayoutUpdated(layout))))
            .unwrap();
    }
}

/// LAYOUT() a wrapper around raw.
pub fn layout_function(event_sender: EventSender) -> impl Fn(LayoutLocation, rhai::Array) {
    move |location: LayoutLocation, children: rhai::Array| {
        let mut opts = rhai::Map::new();
        opts.insert("location".into(), rhai::Dynamic::from(location));
        opts.insert("layout".into(), rhai::Dynamic::from_array(children));

        let raw = layout_function_raw(event_sender.clone());
        raw(opts)
    }
}

/// LAYOUT() a wrapper around raw & can pass extra opts.
pub fn layout_function_ext(event_sender: EventSender) -> impl Fn(rhai::Array, rhai::Map) {
    move |children: rhai::Array, ext_opts: rhai::Map| {
        let mut opts = rhai::Map::new();
        opts.insert("layout".into(), rhai::Dynamic::from_array(children));
        opts.extend(ext_opts);

        let raw = layout_function_raw(event_sender.clone());
        raw(opts)
    }
}
