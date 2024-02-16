use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::utils::default_effects;

use super::layout::AxisSizingMode;
use super::{
    blend_mode::BlendMode,
    effect::{Effect, EffectType},
    export_settings::ExportSetting,
    layout::{
        LayoutAlign, LayoutAlignContent, LayoutAlignItems, LayoutConstraint, LayoutGrid,
        LayoutMode, LayoutPositioning, LayoutSizingMode, LayoutWrap,
    },
    node_common::NodeCommon,
    overflow_direction::OverflowDirection,
    paint::Paint,
    rectangle::Rectangle,
    stroke_align::StrokeAlign,
    stroke_weights::StrokeWeights,
    styles::StyleType,
    transform::Transform,
    vector::Vector,
};

// Only user action pseudo-classes
const PSEUDO_CLASSES: [&str; 5] = ["hover", "active", "focus", "focus-visible", "focus-within"];

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
    #[serde(default)]
    pub layout_align: LayoutAlign,
    #[serde(default)]
    pub layout_grow: f32,
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
    #[serde(default)]
    pub layout_sizing_horizontal: LayoutSizingMode, // HUG, FIXED, FILL
    #[serde(default)]
    pub layout_sizing_vertical: LayoutSizingMode, // HUG, FIXED, FILL
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
    pub layout_grids: Option<Vec<LayoutGrid>>,
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

    pub fn is_variant(&self) -> bool {
        self.node.name.contains("=")
    }

    // name -> .name
    // property=value -> [property="value"]
    // property=pseudo -> :pseudo
    // property=value;pseudo -> [property="value"]:pseudo
    // property=value;class -> [property="value"].class
    pub fn get_classes(&self) -> String {
        let name = self.node.name.clone();
        let mut current_classes = String::new();

        // Variant attributes/pseudo-classes
        if name.contains(",") {
            let variants: Vec<&str> = name.split(", ").collect();
            let mut pseudo = String::new();

            for variant in variants {
                current_classes.push_str(&self.create_variant_classes(variant));
                pseudo.push_str(&self.create_pseudo_classes(variant));
            }

            current_classes.push_str(&pseudo);
        } else if name.contains("=") {
            current_classes.push_str(&self.create_variant_classes(&name));
            current_classes.push_str(&self.create_pseudo_classes(&name));
        } else {
            current_classes = format!(" .{}", self.get_name());
        }

        current_classes
    }

    fn create_variant_classes(&self, variant: &str) -> String {
        if let Some((first, last)) = variant.split_once("=") {
            let attribute = first.to_case(Case::Kebab);
            let value = last.to_case(Case::Kebab);

            if let Some((val, second)) = value.split_once(";") {
                let val = val.to_case(Case::Kebab);
                let cl: String = if !PSEUDO_CLASSES.contains(&second) {
                    format!(".{}", second.to_case(Case::Kebab))
                } else {
                    String::new()
                };
                return format!("[{attribute}=\"{val}\"]{cl}");
            } else if !value.eq("default") && !PSEUDO_CLASSES.contains(&value.as_str()) {
                return format!("[{attribute}=\"{value}\"]");
            }
        }
        String::new()
    }

    fn create_pseudo_classes(&self, variant: &str) -> String {
        if let Some((_, last)) = variant.split_once("=") {
            let value = last.to_case(Case::Kebab);

            if let Some((_, pseudo)) = value.split_once(";") {
                let pseudo = pseudo.to_case(Case::Kebab);
                return format!(":{pseudo}");
            } else if !value.eq("default") && PSEUDO_CLASSES.contains(&value.as_str()) {
                return format!(":{value}");
            }
        }
        String::new()
    }

    pub fn css(&self, parent: Frame) -> BTreeMap<String, String> {
        let mut rules: BTreeMap<String, String> = BTreeMap::new();

        if !self.node.visible {
            rules.insert("display".to_string(), "none".to_string());
        }

        if self.clips_content {
            rules.insert("overflow".to_string(), "hidden".to_string());
        }

        if !self.sizes(parent.clone()).is_empty() {
            for (key, value) in self.sizes(parent.clone()).iter() {
                rules.insert(key.to_string(), value.to_string());
            }
        }

        if self.layout_mode.is_auto_layout() {
            if self.node.visible {
                rules.insert("display".to_string(), "flex".to_string());
            }

            if !self.layout_wrap().is_empty() {
                rules.insert("flex-wrap".to_string(), self.layout_wrap());
            }

            if self.layout_mode.is_vertical() {
                rules.insert("flex-direction".to_string(), "column".to_string());
            }

            if !self.alignment().is_empty() {
                for (key, value) in self.alignment().iter() {
                    rules.insert(key.to_string(), value.to_string());
                }
            }

            if !self.gap().is_empty() {
                rules.insert("gap".to_string(), self.gap());
            }

            if !self.padding().is_empty() {
                rules.insert("padding".to_string(), self.padding());
            }
        }

        // Rotation only works well for 90 * n degrees, for other values like 45deg figma changes the sizes of width and height.
        if !self.rotation().is_empty() {
            rules.insert("transform".to_string(), self.rotation());
        }

        if !self.border_radius().is_empty() {
            rules.insert("border-radius".to_string(), self.border_radius());
        }

        if !self.border().is_empty() {
            for (key, value) in self.border().iter() {
                rules.insert(key.to_string(), value.to_string());
            }
        }

        if !self.background().is_empty() {
            rules.insert("background".to_string(), self.background());
        }

        if !self.box_shadow().is_empty() {
            rules.insert("box-shadow".to_string(), self.box_shadow());
        }

        if !self.blur().is_empty() {
            rules.insert("filter".to_string(), self.blur());
        }

        if !self.background_blur().is_empty() {
            rules.insert("backdrop-filter".to_string(), self.background_blur());
        }

        rules
    }

    fn width(&self) -> String {
        match self.absolute_bounding_box {
            Some(rec) => match rec.width {
                Some(w) => format!("{}px", w),
                None => String::new(),
            },
            None => String::new(),
        }
    }

    fn height(&self) -> String {
        match self.absolute_bounding_box {
            Some(rec) => match rec.height {
                Some(h) => format!("{}px", h),
                None => String::new(),
            },
            None => String::new(),
        }
    }

    fn border_radius(&self) -> String {
        if !self.corner_radius().is_empty() {
            return self.corner_radius();
        }

        if !self.rectangle_corner_radii().is_empty() {
            return self.rectangle_corner_radii();
        }

        return String::new();
    }

    fn background(&self) -> String {
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

    fn rotation(&self) -> String {
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

    fn border(&self) -> HashMap<String, String> {
        // TODO: when multiple colours and sizes convert into "border-width", "border-color" and "border-style"
        if !self.border_individual().is_empty() {
            return self.border_individual();
        } else if !self.border_all().is_empty() {
            return self.border_all();
        }

        return HashMap::new();
    }

    fn box_shadow(&self) -> String {
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

    fn blur(&self) -> String {
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

    fn background_blur(&self) -> String {
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

    fn alignment(&self) -> HashMap<String, String> {
        let mut styles: HashMap<String, String> = HashMap::new();

        let align = match self.counter_axis_align_items {
            LayoutAlignItems::Center => "center".to_string(),
            LayoutAlignItems::Max => "flex-end".to_string(),
            LayoutAlignItems::SpaceBetween => String::new(), // align items does not have space between
            LayoutAlignItems::Baseline => "baseline".to_string(),
            _ => "flex-start".to_string(), // Default LayoutAlignItems::Min
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

    fn layout_wrap(&self) -> String {
        match self.layout_wrap {
            LayoutWrap::Wrap => "wrap".to_string(),
            LayoutWrap::NoWrap => String::new(),
        }
    }

    fn gap(&self) -> String {
        match self.item_spacing {
            Some(x) => format!("{}px", x),
            None => String::new(),
        }
    }

    fn padding(&self) -> String {
        // TODO: remove px for when value is 0
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

    fn sizes(&self, parent: Frame) -> HashMap<String, String> {
        let mut styles: HashMap<String, String> = HashMap::new();

        if let Some(x) = self.min_width {
            styles.insert("min-width".to_string(), format!("{}px", x));
        }

        if let Some(x) = self.max_width {
            styles.insert("max-width".to_string(), format!("{}px", x));
        }

        if let Some(x) = self.min_height {
            styles.insert("min-height".to_string(), format!("{}px", x));
        }

        if let Some(x) = self.max_height {
            styles.insert("max-height".to_string(), format!("{}px", x));
        }

        if self.layout_mode.is_none() {
            if parent.layout_mode.is_auto_layout() {
                if self.layout_sizing_horizontal.is_fixed() {
                    if !self.width().is_empty() {
                        styles.insert("width".to_string(), self.width());
                    }
                }

                // TODO: this sometimes get's added when not needed, check cases for shrink
                // cmp-53, cmp-54 are adding when it does not need
                // so far adding this when is not necessary it does not seem to impact the styles in the browser
                if self.layout_grow == 0.0 {
                    styles.insert("flex-shrink".to_string(), "0".to_string());
                }

                if self.layout_sizing_horizontal.is_fill() {
                    if self.layout_align.is_stretch() {
                        styles.insert("align-self".to_string(), "stretch".to_string());
                    } else {
                        styles.insert("flex".to_string(), "1 0 0".to_string());
                    }
                }

                if self.layout_sizing_vertical.is_fixed() {
                    if !self.height().is_empty() {
                        styles.insert("height".to_string(), self.height());
                    }
                }

                if self.layout_sizing_vertical.is_fill() {
                    if self.layout_grow == 1.0 {
                        styles.insert("flex".to_string(), "1 0 0".to_string());
                    } else {
                        styles.insert("align-self".to_string(), "stretch".to_string());
                    }
                }
            } else {
                if !self.width().is_empty() {
                    styles.insert("width".to_string(), self.width());
                }
                if !self.height().is_empty() {
                    styles.insert("height".to_string(), self.height());
                }
            }
        } else if self.layout_mode.is_auto_layout() {
            if self.layout_sizing_horizontal.is_hug() {
                styles.insert("width".to_string(), "fit-content".to_string());
            }
            if self.layout_sizing_horizontal.is_fixed() {
                styles.insert("width".to_string(), self.width());
            }
            if self.layout_sizing_horizontal.is_fill() {
                styles.insert("width".to_string(), "100%".to_string());
            }

            if self.layout_sizing_vertical.is_hug() {
                styles.insert("height".to_string(), "fit-content".to_string());
            }
            if self.layout_sizing_vertical.is_fixed() {
                styles.insert("height".to_string(), self.height());
            }
            if self.layout_sizing_vertical.is_fill() {
                styles.insert("height".to_string(), "100%".to_string());
            }
        }

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
