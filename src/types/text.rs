use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextCase {
    #[default]
    Original,
    Upper,
    Lower,
    Title,
    SmallCaps,
    SmallCapsForced,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextDecoration {
    #[default]
    None,
    Strikethrough,
    Underline,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAutoResize {
    #[default]
    None,
    Height,
    WidthAndHeight,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAlignHorizontal {
    #[default]
    Left,
    Right,
    Center,
    Justified,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAlignVertical {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextTruncation {
    #[default]
    Disabled,
    Ending,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LineHeightUnit {
    Pixels,
    #[serde(rename = "FONT_SIZE_%")]
    FontSizePercentage,
    #[serde(rename = "INTRINSIC_%")]
    IntrinsicPercentage,
}