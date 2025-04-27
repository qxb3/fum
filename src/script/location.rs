/// Where in the terminal should fum be positioned.
#[derive(Debug, Clone, PartialEq)]
pub enum UiLocation {
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

impl UiLocation {
    pub fn resolve_location(&self) -> (taffy::AlignItems, taffy::JustifyContent) {
        match self {
            UiLocation::Top => (taffy::AlignItems::Start, taffy::JustifyContent::Center),
            UiLocation::TopLeft => (taffy::AlignItems::Start, taffy::JustifyContent::Start),
            UiLocation::TopRight => (taffy::AlignItems::Start, taffy::JustifyContent::End),

            UiLocation::Bottom => (taffy::AlignItems::End, taffy::JustifyContent::Center),
            UiLocation::BottomLeft => (taffy::AlignItems::End, taffy::JustifyContent::Start),
            UiLocation::BottomRight => (taffy::AlignItems::End, taffy::JustifyContent::End),

            UiLocation::Left => (taffy::AlignItems::Center, taffy::JustifyContent::Start),
            UiLocation::Right => (taffy::AlignItems::Center, taffy::JustifyContent::End),
            UiLocation::Center => (taffy::AlignItems::Center, taffy::JustifyContent::Center),
        }
    }
}
