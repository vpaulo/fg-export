use serde::{Deserialize, Serialize};

use super::colour::Colour;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerticalLayoutConstraintValue {
    #[default]
    Top,
    Bottom,
    Center,
    TopBottom,
    Scale,
}
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HorizontalLayoutConstraintValue {
    #[default]
    Left,
    Right,
    Center,
    LeftRight,
    Scale,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
pub struct LayoutConstraint {
    pub vertical: VerticalLayoutConstraintValue,
    pub horizontal: HorizontalLayoutConstraintValue,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutAlign {
    #[default]
    Inherit,
    Stretch,
    Min,
    Center,
    Max,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutMode {
    #[default]
    None,
    Horizontal,
    Vertical,
}

impl LayoutMode {
    pub fn is_none(&self) -> bool {
        match self {
            LayoutMode::None => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            LayoutMode::Vertical => true,
            _ => false,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            LayoutMode::Horizontal => true,
            _ => false,
        }
    }

    pub fn is_auto_layout(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutSizingMode {
    Hug,
    Fixed,
    Auto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AxisSizingMode {
    Fixed,
    #[default]
    Auto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutAlignItems {
    #[default]
    Min,
    Center,
    Max,
    SpaceBetween,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutWrap {
    Wrap,
    #[default]
    NoWrap,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutAlignContent {
    SpaceBetween,
    #[default]
    Auto,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutPositioning {
    Absolute,
    #[default]
    Auto,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutGridPattern {
    Columns,
    Rows,
    Grid,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayoutGridAlignment {
    Min,
    Stretch,
    Center,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LayoutGrid {
    pub pattern: LayoutGridPattern,
    pub section_size: f32,
    pub visible: bool,
    pub color: Colour,
    pub alignment: LayoutGridAlignment,
    pub gutter_size: f32,
    pub offset: f32,
    pub count: i32,
}
