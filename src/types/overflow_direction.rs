use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverflowDirection {
    #[default]
    None,
    HorizontalScrolling,
    VerticalScrolling,
    HorizontalAndVerticalScrolling,
}

impl OverflowDirection {
    pub fn scrolls_horizontal(&self) -> bool {
        match self {
            OverflowDirection::HorizontalScrolling => true,
            OverflowDirection::HorizontalAndVerticalScrolling => true,
            _ => false,
        }
    }
    pub fn scrolls_vertical(&self) -> bool {
        match self {
            OverflowDirection::VerticalScrolling => true,
            OverflowDirection::HorizontalAndVerticalScrolling => true,
            _ => false,
        }
    }
    pub fn scrolls(&self) -> bool {
        match self {
            OverflowDirection::None => false,
            _ => true,
        }
    }
}
