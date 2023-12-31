use super::{
    blend_mode::BlendMode,
    easing_type::EasingType,
    effect::Effect,
    export_settings::ExportSetting,
    layout::{LayoutConstraint, LayoutSizingMode},
    node_common::NodeCommon,
    paint::Paint,
    path::Path,
    rectangle::Rectangle,
    stroke_align::StrokeAlign,
    styles::StyleType,
    transform::Transform,
    vector::Vector,
};
use crate::utils::default_opacity;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub styles: Option<HashMap<StyleType, String>>,
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

    pub fn sizes(&self) -> HashMap<String, String> {
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
}
