use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::{default_effects, default_opacity};

use super::{
    blend_mode::BlendMode,
    constraint::Constraint,
    effect::Effect,
    layout::{
        LayoutAlign, LayoutAlignContent, LayoutAlignItems, LayoutConstraint, LayoutGrid,
        LayoutMode, LayoutPositioning, LayoutSizingMode, LayoutWrap,
    },
    node::NodeCommon,
    paint::Paint,
    rectangle::Rectangle,
    styles::StyleType,
    transform::Transform,
    vector::Vector,
};

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
    pub stroke_dashes: Option<Vec<f32>>,
    pub corner_radius: Option<f32>,
    pub rectangle_corner_radii: Option<[f32; 4]>,
    pub export_settings: Option<Vec<ExportSetting>>,
    pub blend_mode: Option<BlendMode>,
    #[serde(default)]
    pub preserve_ratio: bool,
    pub constraints: LayoutConstraint,
    pub layout_align: Option<LayoutAlign>,
    pub opacity: Option<f32>,
    pub absolute_bounding_box: Option<Rectangle>,
    // pub absolute_render_bounds: Option<Rectangle>, // this returns the bounds of the frame regarding the file, so it's not needed
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
    pub layout_wrap: Option<LayoutWrap>,
    pub primary_axis_sizing_mode: Option<LayoutSizingMode>, // FIXED, AUTO
    pub counter_axis_sizing_mode: Option<LayoutSizingMode>, // FIXED, AUTO
    pub primary_axis_align_items: Option<LayoutAlignItems>, // MIN, CENTER, MAX, SPACE_BETWEEN
    pub counter_axis_align_items: Option<LayoutAlignItems>, // MIN, CENTER, MAX,
    pub counter_axis_align_content: Option<LayoutAlignContent>, // AUTO SPACE_BETWEEN
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    // #[serde(default)]
    // pub horizontal_padding: f32,
    // #[serde(default)]
    // pub vertical_padding: f32,
    pub item_spacing: Option<f32>,
    pub counter_axis_spacing: Option<f32>,
    pub layout_positioning: Option<LayoutPositioning>, // AUTO ABSOLUTE
    pub item_reverse_z_index_boolean: Option<bool>,
    pub strokes_included_in_layout: Option<bool>,
    layout_grids: Option<Vec<LayoutGrid>>,
    pub overflow_direction: Option<OverflowDirection>,
    #[serde(default = "default_effects")]
    pub effects: Vec<Effect>,
    pub is_mask: Option<bool>,
    // #[serde(default)]
    // pub is_mask_outline: bool,
    pub styles: Option<HashMap<StyleType, String>>,
}

impl Frame {
    pub fn get_name(&self) -> String {
        self.node.name.to_case(Case::Kebab)
    }

    pub fn width(&self) -> String {
        match self.absolute_bounding_box {
            Some(rec) => match rec.width {
                Some(w) => format!("{}px", w),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn height(&self) -> String {
        match self.absolute_bounding_box {
            Some(rec) => match rec.height {
                Some(h) => format!("{}px", h),
                None => "".to_string(),
            },
            None => "".to_string(),
        }
    }

    pub fn border_radius(&self) -> String {
        if !self.corner_radius().is_empty() {
            return self.corner_radius();
        }

        if !self.rectangle_corner_radii().is_empty() {
            return self.rectangle_corner_radii();
        }

        return "".to_string();
    }
    pub fn background(&self) -> String {
        for paint in self.fills.iter() {
            println!("{:?}", paint);
            if paint.visible && paint.data.get_solid().is_some() {
                // TODO: get colours, maybe move this logic to get_solid
                return match paint.data.get_solid() {
                    Some(c) => c.rgba(),
                    None => "".to_string(),
                };
            }
        }

        return "".to_string();
    }

    fn corner_radius(&self) -> String {
        match self.corner_radius {
            Some(x) => format!("{}px", x),
            None => "".to_string(),
        }
    }

    // TODO: implement shorthands for the border radius
    fn rectangle_corner_radii(&self) -> String {
        match self.rectangle_corner_radii {
            Some([top_left, top_right, bottom_right, bottom_left]) => format!(
                "{}px {}px {}px {}px",
                top_left, top_right, bottom_right, bottom_left
            ),
            None => "".to_string(),
        }
    }
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
