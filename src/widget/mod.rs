/// Fum Widget.
#[derive(Debug, PartialEq, Clone)]
pub enum Widget {
    /// A container that can manage and contains another widgets.
    Container { children: Vec<Widget> },

    /// Used for displaying texts.
    Label { text: String },
}
