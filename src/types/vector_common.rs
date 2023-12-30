use super::{
    blend_mode::BlendMode, easing_type::EasingType, effect::Effect, export_settings::ExportSetting,
    layout::LayoutConstraint, node_common::NodeCommon, paint::Paint, path::Path,
    rectangle::Rectangle, stroke_align::StrokeAlign, styles::StyleType, transform::Transform,
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
    pub absolute_bounding_box: Option<Rectangle>,
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
}
