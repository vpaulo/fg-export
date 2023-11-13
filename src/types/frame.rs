use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::{collections::HashMap, os::unix::raw::off_t};

use crate::utils::{default_effects, default_opacity};

use super::layout::AxisSizingMode;
use super::{
    blend_mode::BlendMode,
    constraint::Constraint,
    effect::{self, Effect, EffectType},
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
    pub individual_stroke_weights: Option<StrokeWeights>,
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
    pub layout_sizing_horizontal: Option<LayoutSizingMode>, // HUG, FIXED, FILL
    pub layout_sizing_vertical: Option<LayoutSizingMode>,   // HUG, FIXED, FILL
    #[serde(default)]
    pub layout_wrap: LayoutWrap,
    #[serde(default)]
    pub primary_axis_sizing_mode: AxisSizingMode, // FIXED, AUTO -> This property is only applicable for auto-layout frames
    #[serde(default)]
    pub counter_axis_sizing_mode: AxisSizingMode, // FIXED, AUTO -> This property is only applicable for auto-layout frames
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
                None => String::new(),
            },
            None => String::new(),
        }
    }

    pub fn height(&self) -> String {
        match self.absolute_bounding_box {
            Some(rec) => match rec.height {
                Some(h) => format!("{}px", h),
                None => String::new(),
            },
            None => String::new(),
        }
    }

    pub fn border_radius(&self) -> String {
        if !self.corner_radius().is_empty() {
            return self.corner_radius();
        }

        if !self.rectangle_corner_radii().is_empty() {
            return self.rectangle_corner_radii();
        }

        return String::new();
    }

    pub fn background(&self) -> String {
        for paint in self.fills.iter() {
            if paint.visible && paint.data.get_solid().is_some() {
                // TODO: get colours, maybe move this logic to get_solid
                // TODO: build string for when there's multiple backgrounds
                // Multiple Solid backgrounds converts to a linear gradient, for now we select the first one passing the condition.
                return match paint.data.get_solid() {
                    Some(c) => c.rgba(),
                    None => String::new(),
                };
            }
        }

        return String::new();
    }

    pub fn rotation(&self) -> String {
        match self.rotation {
            // If None or zero return empty string.
            Some(r) => {
                let degrees = f32::to_degrees(r);
                if degrees.round() != 0.0 {
                    format!("rotate({:.0}deg)", degrees)
                } else {
                    String::new()
                }
            }
            None => String::new(),
        }
    }

    pub fn border(&self) -> HashMap<String, String> {
        // TODO: when multiple colours and sizes convert into "border-width", "border-color" and "border-style"
        if !self.border_individual().is_empty() {
            return self.border_individual();
        } else if !self.border_all().is_empty() {
            return self.border_all();
        }

        return HashMap::new();
    }

    pub fn box_shadow(&self) -> String {
        let effect_list: Vec<String> = self
            .effects
            .iter()
            .filter(|x| x.visible)
            .map(|e| match e.effect_type {
                EffectType::InnerShadow => format!("inset {}", Frame::shadow(e)),
                EffectType::DropShadow => Frame::shadow(e),
                _ => String::new(),
            })
            .collect();

        effect_list.join(", ")
    }

    pub fn blur(&self) -> String {
        let filter_list: Vec<String> = self
            .effects
            .iter()
            .filter(|x| x.visible)
            .map(|e| match e.effect_type {
                EffectType::LayerBlur => format!("blur({:0}px)", e.radius),
                _ => String::new(),
            })
            .collect();

        match filter_list.first() {
            Some(x) => x.to_string(),
            None => String::new(),
        }
    }

    pub fn background_blur(&self) -> String {
        let filter_list: Vec<String> = self
            .effects
            .iter()
            .filter(|x| x.visible)
            .map(|e| match e.effect_type {
                EffectType::BackgroundBlur => format!("blur({:0}px)", e.radius),
                _ => String::new(),
            })
            .collect();

        match filter_list.first() {
            Some(x) => x.to_string(),
            None => String::new(),
        }
    }

    pub fn alignment(&self) -> HashMap<String, String> {
        let mut styles: HashMap<String, String> = HashMap::new();

        // TODO: keeping this for we pass the parent just in case
        // let align_items = if self.layout_mode.is_vertical() {
        //     &self.primary_axis_align_items
        // } else {
        //     &self.counter_axis_align_items
        // };
        // let justify_content = if self.layout_mode.is_vertical() {
        //     &self.counter_axis_align_items
        // } else {
        //     &self.primary_axis_align_items
        // };

        let align = match self.counter_axis_align_items {
            LayoutAlignItems::Center => "center".to_string(),
            LayoutAlignItems::Max => "flex-end".to_string(),
            LayoutAlignItems::SpaceBetween => String::new(), // align items does not have space between
            _ => "flex-start".to_string(),                   // Default LayoutAlignItems::Min
        };

        let justify = match self.primary_axis_align_items {
            LayoutAlignItems::Center => "center".to_string(),
            LayoutAlignItems::Max => "flex-end".to_string(),
            LayoutAlignItems::SpaceBetween => "space-between".to_string(),
            _ => "flex-start".to_string(), // Default LayoutAlignItems::Min
        };

        let content = match self.counter_axis_align_content {
            LayoutAlignContent::SpaceBetween => "space-between".to_string(),
            LayoutAlignContent::Auto => String::new(),
        };

        if !align.is_empty() {
            styles.insert("align-items".to_string(), align);
        }

        if !justify.is_empty() {
            styles.insert("justify-content".to_string(), justify);
        }

        if !content.is_empty() && !self.layout_wrap().is_empty() {
            styles.insert("align-content".to_string(), content);
        }

        styles
    }

    pub fn layout_wrap(&self) -> String {
        match self.layout_wrap {
            LayoutWrap::Wrap => "wrap".to_string(),
            LayoutWrap::NoWrap => String::new(),
        }
    }

    pub fn gap(&self) -> String {
        match self.item_spacing {
            Some(x) => format!("{}px", x),
            None => String::new(),
        }
    }

    pub fn padding(&self) -> String {
        let top = self.padding_top;
        let right = self.padding_right;
        let bottom = self.padding_bottom;
        let left = self.padding_left;

        if top == bottom && right == left && top == right {
            format!("{top}px")
        } else if top == bottom && right == left {
            format!("{top}px {right}px")
        } else if right == left {
            format!("{top}px {right}px {bottom}px")
        } else {
            format!("{top}px {right}px {bottom}px {left}px")
        }
    }

    pub fn sizes(&self) -> HashMap<String, String> {
        let mut styles: HashMap<String, String> = HashMap::new();

        // TODO: there's a lot of matches that repeat the same logic, maybe extract the match to a utility function??
        let min_width: String = match self.min_width {
            Some(x) => format!("{}px", x),
            None => String::new(),
        };
        let max_width: String = match self.max_width {
            Some(x) => format!("{}px", x),
            None => String::new(),
        };
        let min_height: String = match self.min_height {
            Some(x) => format!("{}px", x),
            None => String::new(),
        };
        let max_height: String = match self.max_height {
            Some(x) => format!("{}px", x),
            None => String::new(),
        };

        if !min_width.is_empty() {
            styles.insert("min-width".to_string(), min_width);
        }

        if !max_width.is_empty() {
            styles.insert("max-width".to_string(), max_width);
        }

        if !min_height.is_empty() {
            styles.insert("min-height".to_string(), min_height);
        }

        if !max_height.is_empty() {
            styles.insert("max-height".to_string(), max_height);
        }

        // TODO: width and height

        styles
    }

    fn corner_radius(&self) -> String {
        match self.corner_radius {
            Some(x) => format!("{}px", x),
            None => String::new(),
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
            None => String::new(),
        }
    }

    fn border_style(&self) -> String {
        if self.stroke_dashes.is_some() {
            return "dashed".to_string();
        }

        "solid".to_string()
    }

    fn border_colour(&self) -> String {
        for paint in self.strokes.iter() {
            if paint.visible && paint.data.get_solid().is_some() {
                // TODO: Same as background
                return match paint.data.get_solid() {
                    Some(c) => c.rgba(),
                    None => String::new(),
                };
            }
        }

        String::new()
    }

    fn border_all(&self) -> HashMap<String, String> {
        let mut borders: HashMap<String, String> = HashMap::new();

        let width = match self.stroke_weight {
            Some(x) => format!("{}px", x),
            None => String::new(),
        };

        let style = self.border_style();
        let colour = self.border_colour();

        if !width.is_empty() & !colour.is_empty() {
            borders.insert("border".to_string(), format!("{width} {style} {colour}"));
            borders
        } else {
            HashMap::new()
        }
    }

    fn border_individual(&self) -> HashMap<String, String> {
        let mut borders: HashMap<String, String> = HashMap::new();

        let style = self.border_style();
        let colour = self.border_colour();

        match self.individual_stroke_weights {
            Some(border) => {
                if border.top > 0.0 && !colour.is_empty() {
                    borders.insert(
                        "border-top".to_string(),
                        format!("{}px {} {}", border.top, style, colour),
                    );
                }
                if border.right > 0.0 && !colour.is_empty() {
                    borders.insert(
                        "border-right".to_string(),
                        format!("{}px {} {}", border.right, style, colour),
                    );
                }
                if border.bottom > 0.0 && !colour.is_empty() {
                    borders.insert(
                        "border-bottom".to_string(),
                        format!("{}px {} {}", border.bottom, style, colour),
                    );
                }
                if border.left > 0.0 && !colour.is_empty() {
                    borders.insert(
                        "border-left".to_string(),
                        format!("{}px {} {}", border.left, style, colour),
                    );
                }

                borders
            }
            None => HashMap::new(),
        }
    }

    fn shadow(effect: &Effect) -> String {
        let Effect {
            offset,
            spread,
            radius,
            color,
            ..
        } = effect;
        let x = offset.x();
        let y = offset.y();
        let rgba = color.rgba();
        format!("{x:0}px {y:0}px {radius:0}px {spread:0}px {rgba}")
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
