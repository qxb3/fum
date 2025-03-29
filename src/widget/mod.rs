/// Fum Widget.
#[derive(Debug, PartialEq, Clone)]
pub enum Widget {
    /// A container that can manage and contains another widgets.
    Container {
        /// Container children.
        children: Vec<Widget>,

        /// Container total width.
        width: u16,

        /// Container total height.
        height: u16,
    },

    /// Used for displaying texts.
    Label {
        /// Label text content.
        text: String,

        /// Label width.
        width: u16,

        /// Label height.
        height: u16,
    },
}

impl Widget {
    /// Get the width of the widget.
    pub fn get_width(&self) -> u16 {
        match self {
            Widget::Container {
                width,
                ..
            } => *width,
            Widget::Label {
                width,
                ..
            } => *width,
        }
    }

    /// Get the height of the widget.
    pub fn get_height(&self) -> u16 {
        match self {
            Widget::Container {
                height,
                ..
            } => *height,
            Widget::Label {
                height,
                ..
            } => *height,
        }
    }
}
