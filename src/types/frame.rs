use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f32::consts::PI;

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
    pub rotation: Option<f32>,
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
                // TODO: build string for when there's multiple backgrounds
                // Multiple Solid backgrounds converts to a linear gradient, for now we select the first one passing the condition.
                return match paint.data.get_solid() {
                    Some(c) => c.rgba(),
                    None => "".to_string(),
                };
            }
        }

        return "".to_string();
    }

    pub fn rotation(&self) -> String {
        match self.rotation {
            // If None or zero return empty string.
            Some(r) => {
                if ((r / PI) * 180.0).round() != 0.0 {
                    format!("rotate({:.0}deg)", (r / PI) * 180.0)
                } else {
                    "".to_string()
                }
            }
            None => "".to_string(),
        }
    }

    pub fn border(&self) -> String {
        // TODO: a lot of options for the border :/
        // "strokes": [
        //       {
        //         "visible": true,
        //         "opacity": 1.0,
        //         "type": "SOLID",
        //         "color": {
        //           "a": 1.0,
        //           "r": 0.0,
        //           "g": 0.0,
        //           "b": 0.0
        //         }
        //       }
        //     ],
        //     "strokeWeight": 1.0,
        //     "strokeAlign": "INSIDE",
        //     "strokeDashes": null,
        //     "cornerRadius": null,
        // match self.corner_radius {
        //     Some(x) => format!("{}px", x),
        //     None => "".to_string(),
        // }
        return "".to_string();
    }

    fn corner_radius(&self) -> String {
        match self.corner_radius {
            Some(x) => format!("{}px", x),
            None => "".to_string(),
        }
    }

    fn rectangle_corner_radii(&self) -> String {
        match self.rectangle_corner_radii {
            Some([top_left, top_right, bottom_right, bottom_left]) => {
                if top_left == bottom_right && top_right == bottom_left {
                    format!("{}px {}px", top_left, top_right)
                } else if top_right == bottom_left {
                    format!("{}px {}px {}px", top_left, top_right, bottom_right)
                } else {
                    format!(
                        "{}px {}px {}px {}px",
                        top_left, top_right, bottom_right, bottom_left
                    )
                }
            }
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn rectangle_corner_radii() {
        /* top-left | top-right | bottom-right | bottom-left */
        assert_eq!(
            Frame {
                rectangle_corner_radii: Some([1.0, 2.0, 3.0, 4.0]),
                ..Frame::default()
            }
            .rectangle_corner_radii(),
            "1px 2px 3px 4px"
        );

        // shorthands

        /* top-left-and-bottom-right | top-right-and-bottom-left */
        assert_eq!(
            Frame {
                rectangle_corner_radii: Some([1.0, 2.0, 1.0, 2.0]),
                ..Frame::default()
            }
            .rectangle_corner_radii(),
            "1px 2px"
        );
        /* top-left | top-right-and-bottom-left | bottom-right */
        assert_eq!(
            Frame {
                rectangle_corner_radii: Some([1.0, 2.0, 3.0, 2.0]),
                ..Frame::default()
            }
            .rectangle_corner_radii(),
            "1px 2px 3px"
        );
    }

    #[test]
    fn rotation() {
        assert_eq!(
            Frame {
                rotation: Some(-1.5707964),
                ..Frame::default()
            }
            .rotation(),
            "rotate(-90deg)"
        );
        assert_eq!(
            Frame {
                rotation: Some(-0.7853982),
                ..Frame::default()
            }
            .rotation(),
            "rotate(-45deg)"
        );
        assert_eq!(
            Frame {
                rotation: Some(-5.551115e-17), // This number is very close to 0, so we will assume that it is 0
                ..Frame::default()
            }
            .rotation(),
            ""
        );
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
