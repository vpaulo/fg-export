use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{
    blend_mode::BlendMode,
    effect::Effect,
    layout::LayoutConstraint,
    paint::Paint,
    rectangle::Rectangle,
    styles::{StyleType},
    transform::Transform,
    vector::Vector,
    easing_type::EasingType,
    path::Path,
    node_common::NodeCommon,
    export_settings::ExportSetting,
    stroke_align::StrokeAlign
};
use crate::utils::default_opacity;

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
