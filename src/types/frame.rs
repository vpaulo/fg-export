use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::utils::{default_opacity, default_effects};

use super::{node::NodeCommon, paint::Paint, blend_mode::BlendMode, vector::Vector, transform::Transform, layout::{LayoutConstraint, LayoutAlign, LayoutMode, LayoutSizingMode, LayoutAlignItems, LayoutWrap, LayoutAlignContent, LayoutPositioning, LayoutGrid}, rectangle::Rectangle, styles::StyleType, effect::Effect, constraint::Constraint};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    #[serde(flatten)]
    pub node: NodeCommon,
    #[serde(default)]
    pub fills: Vec<Paint>,
    #[serde(default)]
    pub strokes: Vec<Paint>,
    pub stroke_weight: Option<f32>,
    pub stroke_align: Option<StrokeAlign>,
    #[serde(default)]
    pub stroke_dashes: Vec<f32>,
    pub corner_radius: Option<f32>,
    pub rectangle_corner_radii: Option<[f32; 4]>,
    pub export_settings: Option<Vec<ExportSetting>>,
    pub blend_mode: Option<BlendMode>,
    #[serde(default)]
    pub preserve_ratio: bool,
    pub constraints: LayoutConstraint,
    pub layout_align: Option<LayoutAlign>,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    pub absolute_bounding_box: Option<Rectangle>,
    pub absolute_render_bounds: Option<Rectangle>,
    pub size: Option<Vector>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub relative_transform: Option<Transform>,
    pub clips_content: bool,
    #[serde(default)]
    pub layout_mode: LayoutMode,
    pub layout_sizing_horizontal: Option<LayoutSizingMode>, 
    pub layout_sizing_vertical: Option<LayoutSizingMode>,
    #[serde(default)]
    pub layout_wrap: LayoutWrap,
    #[serde(default)]
    pub primary_axis_sizing_mode: LayoutSizingMode, // FIXED, AUTO
    #[serde(default)]
    pub counter_axis_sizing_mode: LayoutSizingMode, // FIXED, AUTO
    #[serde(default)]
    pub primary_axis_align_items: LayoutAlignItems, // MIN, CENTER, MAX, SPACE_BETWEEN
    #[serde(default)]
    pub counter_axis_align_items: LayoutAlignItems, // MIN, CENTER, MAX,
    #[serde(default)]
    pub counter_axis_align_content: LayoutAlignContent, // AUTO SPACE_BETWEEN
    #[serde(default)]
    pub padding_left: f32,
    #[serde(default)]
    pub padding_right: f32,
    #[serde(default)]
    pub padding_top: f32,
    #[serde(default)]
    pub padding_bottom: f32,
    #[serde(default)]
    pub horizontal_padding: f32,
    #[serde(default)]
    pub vertical_padding: f32,
    #[serde(default)]
    pub item_spacing: f32,
    #[serde(default)]
    pub counter_axis_spacing: f32,
    #[serde(default)]
    pub layout_positioning: LayoutPositioning, // AUTO ABSOLUTE
    #[serde(default)]
    pub item_reverse_z_index_boolean: bool,
    #[serde(default)]
    pub strokes_included_in_layout: bool,
    #[serde(default)]
    layout_grids: Vec<LayoutGrid>,
    #[serde(default)]
    pub overflow_direction: OverflowDirection,
    #[serde(default = "default_effects")]
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub is_mask: bool,
    #[serde(default)]
    pub is_mask_outline: bool,
    pub styles: Option<HashMap<StyleType, String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrokeAlign {
    Inside,
    Outside,
    #[default]
    Center,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct StrokeWeights {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExportSetting {
    pub suffix: String,
    pub format: ExportFormat,
    pub constraint: Constraint,
}

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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExportFormat {
    Jpg,
    Png,
    Svg,
}