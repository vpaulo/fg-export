use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::{
    default_line_height_percent, default_list_spacing, default_paragraph_indent,
    default_paragraph_spacing,
};

use super::{
    paint::Paint,
    text::{
        TextAlignHorizontal, TextAlignVertical, TextAutoResize, TextCase, TextDecoration,
        TextTruncation,
    },
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeStyle {
    pub font_family: String,
    pub font_post_script_name: Option<String>,
    #[serde(default = "default_paragraph_spacing")]
    pub paragraph_spacing: f32, // It does not generate any styles
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
    // pub line_height_px: f32,
    // #[serde(default = "default_line_height_percent")]
    // pub line_height_percent: f32,
    #[serde(default = "default_line_height_percent")]
    pub line_height_percent_font_size: f32,
    // pub line_height_unit: LineHeightUnit,
}

impl TypeStyle {
    pub fn line_height(&self) -> f32 {
        if self.line_height_percent_font_size == 0.0 {
            1.0
        } else {
            self.line_height_percent_font_size / 100.0
        }
    }

    pub fn text_align(&self) -> String {
        match self.text_align_horizontal {
            TextAlignHorizontal::Right => "right".to_string(),
            TextAlignHorizontal::Center => "center".to_string(),
            TextAlignHorizontal::Justified => "justify".to_string(),
            _ => String::new(),
        }
    }

    pub fn text_decoration(&self) -> String {
        match self.text_decoration {
            TextDecoration::Strikethrough => "strikethrough".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            _ => String::new(),
        }
    }

    pub fn text_transform(&self) -> String {
        match self.text_case {
            TextCase::Upper => "uppercase".to_string(),
            TextCase::Lower => "lowercase".to_string(),
            TextCase::Title => "capitalize".to_string(),
            _ => String::new(),
        }
    }

    pub fn font_variant(&self) -> String {
        match self.text_case {
            TextCase::SmallCaps => "small-caps".to_string(),
            TextCase::SmallCapsForced => "all-small-caps".to_string(),
            _ => String::new(),
        }
    }
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
