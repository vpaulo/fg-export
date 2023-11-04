use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::{default_paragraph_spacing, default_paragraph_indent, default_list_spacing, default_line_height_percent};

use super::{paint::Paint, text::{TextCase, TextDecoration, TextAutoResize, TextAlignHorizontal, TextAlignVertical, TextTruncation, LineHeightUnit}};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeStyle {
    pub font_family: Option<String>,
    pub font_post_script_name: Option<String>,
    #[serde(default = "default_paragraph_spacing")]
    pub paragraph_spacing: f32,
    #[serde(default = "default_paragraph_indent")]
    pub paragraph_indent: f32,
    #[serde(default = "default_list_spacing")]
    pub list_spacing: f32,
    #[serde(default)]
    pub italic: bool,
    pub font_weight: f32,
    pub font_size: f32,
    #[serde(default)]
    pub text_case: TextCase,
    #[serde(default)]
    pub text_decoration: TextDecoration,
    #[serde(default)]
    pub text_auto_resize: TextAutoResize,
    #[serde(default)]
    pub text_truncation: TextTruncation,
    pub max_lines: Option<f32>,
    #[serde(default)]
    pub text_align_horizontal: TextAlignHorizontal,
    #[serde(default)]
    pub text_align_vertical: TextAlignVertical,
    pub letter_spacing: f32,
    #[serde(default)]
    pub fills: Vec<Paint>,
    // pub hyperlink: Option<Hyperlink>,
    #[serde(default)]
    pub opentype_flags: HashMap<String, u32>,
    pub line_height_px: f32,
    // #[serde(default = "default_line_height_percent")]
    // pub line_height_percent: f32,
    #[serde(default = "default_line_height_percent")]
    pub line_height_percent_font_size: f32,
    pub line_height_unit: LineHeightUnit,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StyleType {
    Fill,
    Text,
    Effect,
    Grid,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Style {
    pub key: String,
    pub name: String,
    pub description: String,
    pub style_type: StyleType,
}
