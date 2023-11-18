use std::collections::HashMap;

use super::{
    blend_mode::BlendMode,
    effect::Effect,
    frame::{ExportSetting, Frame, StrokeAlign},
    layout::LayoutConstraint,
    paint::Paint,
    rectangle::Rectangle,
    styles::{StyleType, TypeStyle},
    transform::Transform,
    vector::Vector,
};
use crate::utils::{self, default_opacity};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasingType {
    EaseIn,
    EaseOut,
    EaseInAndOut,
    Linear,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub path: String,
    pub winding_rule: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct NodeCommon {
    pub id: String,
    pub name: String,
    #[serde(default = "utils::default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub children: Vec<Node>,
}

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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Node {
    DOCUMENT(NodeCommon),
    CANVAS {
        #[serde(flatten)]
        node: NodeCommon,
        // backgroundColor: Colour,
    },
    FRAME(Frame),
    GROUP(Frame),
    VECTOR(VectorCommon),
    BOOLEAN_OPERATION {
        #[serde(flatten)]
        vector: VectorCommon,
        booleanOperation: String,
    },
    STAR(VectorCommon),
    LINE(VectorCommon),
    ELLIPSE(VectorCommon),
    REGULAR_POLYGON(VectorCommon),
    RECTANGLE {
        #[serde(flatten)]
        vector: VectorCommon,
        cornerRadius: Option<f32>,
        rectangleCornerRadii: Option<[f32; 4]>,
    },
    TEXT {
        #[serde(flatten)]
        vector: VectorCommon,
        characters: String,
        style: TypeStyle,
        characterStyleOverrides: Vec<f32>,
    },
    SLICE {
        #[serde(flatten)]
        node: NodeCommon,
        #[serde(default)]
        exportSettings: Vec<ExportSetting>,
        absoluteBoundingBox: Rectangle,
        size: Option<Vector>,
        relativeTransform: Option<Transform>,
    },
    COMPONENT(Frame),
    INSTANCE {
        #[serde(flatten)]
        frame: Frame,
        componentId: String,
    },
}

impl Node {
    pub fn common(&self) -> &NodeCommon {
        match self {
            Node::DOCUMENT(node) => node,
            Node::CANVAS { node, .. } => node,
            Node::FRAME(Frame { node, .. }) => node,
            Node::GROUP(Frame { node, .. }) => node,
            Node::VECTOR(VectorCommon { node, .. }) => node,
            Node::BOOLEAN_OPERATION {
                vector: VectorCommon { node, .. },
                ..
            } => node,
            Node::STAR(VectorCommon { node, .. }) => node,
            Node::LINE(VectorCommon { node, .. }) => node,
            Node::ELLIPSE(VectorCommon { node, .. }) => node,
            Node::REGULAR_POLYGON(VectorCommon { node, .. }) => node,
            Node::RECTANGLE {
                vector: VectorCommon { node, .. },
                ..
            } => node,
            Node::TEXT {
                vector: VectorCommon { node, .. },
                ..
            } => node,
            Node::SLICE { node, .. } => node,
            Node::COMPONENT(Frame { node, .. }) => node,
            Node::INSTANCE {
                frame: Frame { node, .. },
                ..
            } => node,
        }
    }

    pub fn is_component(&self) -> Option<&Frame> {
        match self {
            Node::COMPONENT(frame) => Some(frame),
            _ => None,
        }
    }

    pub fn is_frame(&self) -> Option<&Frame> {
        match self {
            Node::COMPONENT(frame)
            | Node::INSTANCE { frame, .. }
            | Node::FRAME(frame)
            | Node::GROUP(frame) => Some(frame),
            _ => None,
        }
    }
}
