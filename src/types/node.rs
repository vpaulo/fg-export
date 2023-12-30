use super::{
    export_settings::ExportSetting, frame::Frame, node_common::NodeCommon, rectangle::Rectangle,
    styles::TypeStyle, transform::Transform, vector::Vector, vector_common::VectorCommon,
};
use serde::{Deserialize, Serialize};

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
    #[serde(rename_all = "camelCase")]
    BOOLEAN_OPERATION {
        #[serde(flatten)]
        vector: VectorCommon,
        boolean_operation: String,
    },
    STAR(VectorCommon),
    LINE(VectorCommon),
    ELLIPSE(VectorCommon),
    REGULAR_POLYGON(VectorCommon),
    #[serde(rename_all = "camelCase")]
    RECTANGLE {
        #[serde(flatten)]
        vector: VectorCommon,
        corner_radius: Option<f32>,
        rectangle_corner_radii: Option<[f32; 4]>,
    },
    #[serde(rename_all = "camelCase")]
    TEXT {
        #[serde(flatten)]
        vector: VectorCommon,
        characters: String,
        style: TypeStyle,
        character_style_overrides: Vec<f32>,
    },
    #[serde(rename_all = "camelCase")]
    SLICE {
        #[serde(flatten)]
        node: NodeCommon,
        #[serde(default)]
        export_settings: Vec<ExportSetting>,
        absolute_bounding_box: Rectangle,
        size: Option<Vector>,
        relative_transform: Option<Transform>,
    },
    COMPONENT(Frame),
    #[serde(rename_all = "camelCase")]
    INSTANCE {
        #[serde(flatten)]
        frame: Frame,
        component_id: String,
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

    pub fn is_text(&self) -> Option<(&VectorCommon, &TypeStyle)> {
        match self {
            Node::TEXT { vector, style, .. } => Some((vector, style)),
            _ => None,
        }
    }
}
