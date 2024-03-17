use super::{
    blend_mode::BlendMode,
    easing_type::EasingType,
    effect::{Effect, EffectType},
    export_settings::ExportSetting,
    layout::{LayoutConstraint, LayoutSizingMode},
    node_common::NodeCommon,
    paint::Paint,
    path::Path,
    rectangle::Rectangle,
    stroke_align::StrokeAlign,
    styles::TypeStyle,
    text::TextTruncation,
    transform::Transform,
    vector::Vector,
};
use crate::utils::default_opacity;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VectorCommon {
    #[serde(flatten)]
    pub node: NodeCommon,
    pub locked: Option<bool>,
    pub export_settings: Option<Vec<ExportSetting>>,
    pub blend_mode: BlendMode,
    pub preserve_ratio: Option<bool>,
    pub constraints: LayoutConstraint,
    pub transition_node_id: Option<String>,
    pub transition_duration: Option<f32>,
    pub transition_easing: Option<EasingType>,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub absolute_bounding_box: Option<Rectangle>,
    #[serde(default)]
    pub layout_sizing_horizontal: LayoutSizingMode, // HUG, FIXED, FILL
    #[serde(default)]
    pub layout_sizing_vertical: LayoutSizingMode, // HUG, FIXED, FILL
    pub effects: Vec<Effect>,
    pub size: Option<Vector>,
    pub relative_transform: Option<Transform>,
    pub is_mask: Option<bool>,
    pub fills: Vec<Paint>,
    pub fill_geometry: Option<Vec<Path>>,
    #[serde(default)]
    pub strokes: Vec<Paint>,
    pub stroke_weight: Option<f32>,
    pub stroke_align: Option<StrokeAlign>,
    #[serde(default)]
    pub stroke_dashes: Vec<f32>,
    pub stroke_miter_angle: Option<f32>,
    pub stroke_geometry: Option<Vec<Path>>,
    pub styles: Option<HashMap<String, String>>,
}

impl VectorCommon {
    pub fn get_name(&self) -> String {
        self.node.name.to_case(Case::Kebab)
    }

    pub fn text_colour(&self) -> String {
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

    pub fn border_colour(&self) -> String {
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

    pub fn box_shadow(&self) -> String {
        let effect_list: Vec<String> = self
            .effects
            .iter()
            .filter(|x| x.visible)
            .map(|e| match e.effect_type {
                EffectType::InnerShadow => format!("inset {}", VectorCommon::shadow(e)),
                EffectType::DropShadow => VectorCommon::shadow(e),
                _ => String::new(),
            })
            .collect();

        effect_list.join(", ")
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

    pub fn sizes(&self) -> BTreeMap<String, String> {
        let mut styles: BTreeMap<String, String> = BTreeMap::new();

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

        if self.layout_sizing_horizontal.is_fixed() {
            styles.insert("width".to_string(), self.width());
        }
        if self.layout_sizing_horizontal.is_fill() {
            styles.insert("width".to_string(), "100%".to_string());
            // OR
            // styles.insert("flex".to_string(), "1 0 0".to_string());
        }

        if self.layout_sizing_vertical.is_fixed() {
            styles.insert("height".to_string(), self.height());
        }
        if self.layout_sizing_vertical.is_fill() {
            styles.insert("height".to_string(), "100%".to_string());
            // OR
            // styles.insert("display".to_string(), "flex".to_string());
            // styles.insert("flex-direction".to_string(), "column".to_string());
            // styles.insert("justify-content".to_string(), "center".to_string());
            // styles.insert("flex-shrink".to_string(), "0".to_string());
            // styles.insert("align-self".to_string(), "stretch".to_string());
        }

        styles
    }

    pub fn css(&self, style: &TypeStyle) -> BTreeMap<String, String> {
        let mut rules: BTreeMap<String, String> = BTreeMap::new();

        if !self.text_colour().is_empty() {
            rules.insert("color".to_string(), self.text_colour());
        }

        if !style.font_family.is_empty() {
            rules.insert("font-family".to_string(), style.font_family.to_string());
        }

        if style.font_size != 0.0 {
            rules.insert("font-size".to_string(), format!("{:.0}px", style.font_size));
        }

        if style.font_weight != 0.0 {
            rules.insert(
                "font-weight".to_string(),
                format!("{:.0}", style.font_weight),
            );
        }

        if style.line_height() > 0.0 {
            rules.insert(
                "line-height".to_string(),
                format!("{}", style.line_height()),
            );
        }

        if style.letter_spacing != 0.0 {
            rules.insert(
                "letter-spacing".to_string(),
                format!("{:.0}px", style.letter_spacing),
            );
        }

        if !self.sizes().is_empty() {
            for (key, value) in self.sizes().iter() {
                rules.insert(key.to_string(), value.to_string());
            }
        }

        if !style.text_align().is_empty() {
            rules.insert("text-align".to_string(), format!("{}", style.text_align()));
        }

        if !style.text_decoration().is_empty() {
            rules.insert(
                "text-decoration-line".to_string(),
                format!("{}", style.text_decoration()),
            );
        }

        if !style.text_transform().is_empty() {
            rules.insert(
                "text-transform".to_string(),
                format!("{}", style.text_transform()),
            );
        }

        if !style.font_variant().is_empty() {
            rules.insert(
                "font-variant".to_string(),
                format!("{}", style.font_variant()),
            );
        }

        if style.text_truncation == TextTruncation::Ending {
            rules.insert("text-overflow".to_string(), "ellipsis".to_string());

            if let Some(max) = style.max_lines {
                rules.insert("-webkit-box-orient".to_string(), "vertical".to_string());
                rules.insert("-webkit-line-clamp".to_string(), format!("{:.0}", max));
            }
        }

        rules
    }
}
