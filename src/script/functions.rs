use std::vec;

use crate::widget::Widget;

use super::{ScriptErr, ScriptEvent, ScriptEventSender};

/// FUM_UI() function to set or update the ui.
pub fn fum_ui(tx: ScriptEventSender) -> impl Fn(rhai::Array) {
    move |ui_widgets: rhai::Array| {
        let mut widgets: Vec<Widget> = vec![];

        for widget in ui_widgets {
            match widget.try_cast_result::<Widget>() {
                Ok(widget) => widgets.push(widget),
                Err(_) => {
                    tx.send(Err(ScriptErr::InvalidType(
                        "The ui needs to be an array of widgets".into(),
                    )))
                    .unwrap();
                }
            }
        }

        // Send the event that the FUM_UI() function has been called.
        tx.send(Ok(ScriptEvent::UiUpdate(widgets))).unwrap();
    }
}

/// Container() widget function.
pub fn container(tx: ScriptEventSender) -> impl Fn(rhai::Array) -> Widget {
    move |container_children: rhai::Array| -> Widget {
        // Where the container children is stored.
        let mut children: Vec<Widget> = vec![];

        // The total width & height of the container.
        let mut container_width: u16 = 0;
        let mut container_height: u16 = 0;

        for child in container_children {
            match child.try_cast_result::<Widget>() {
                Ok(child) => {
                    // Add the child width & height to the container width & height.
                    container_width += child.get_width();
                    container_height += child.get_height();

                    children.push(child);
                }
                Err(_) => {
                    tx.send(Err(ScriptErr::InvalidType(
                        "The children of the container needs to be a widget".into(),
                    )))
                    .unwrap();
                }
            }
        }

        Widget::Container {
            children,
            width: container_width,
            height: container_height,
        }
    }
}

/// Label() widget function.
pub fn label() -> impl Fn(&str) -> Widget {
    move |text: &str| -> Widget {
        Widget::Label {
            text: text.to_string(),
            width: text.len() as u16,
            height: 1,
        }
    }
}
