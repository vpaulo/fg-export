use serde::{Deserialize, Serialize};
use super::{colour::Colour, frame::{Frame, ExportSetting}, rectangle::Rectangle, vector::Vector, transform::Transform};
use crate::utils;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NodeCommon {
    pub id: String,
    pub name: String,
    #[serde(default = "utils::default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub children: Vec<Node>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum Node {
    DOCUMENT(NodeCommon),
    CANVAS {
        #[serde(flatten)]
        node: NodeCommon,
        backgroundColor: Colour,
    },
    FRAME(Frame),
    GROUP(Frame),
    // VECTOR(VectorNode),
    // BOOLEAN_OPERATION {
    //     #[serde(flatten)]
    //     vector: VectorNode,
    //     booleanOperation: String,
    // },
    // STAR(VectorNode),
    // LINE(VectorNode),
    // ELLIPSE(VectorNode),
    // REGULAR_POLYGON(VectorNode),
    // RECTANGLE {
    //     #[serde(flatten)]
    //     vector: VectorNode,
    //     cornerRadius: Option<f32>,
    //     #[serde(default)]
    //     rectangleCornerRadii: Vec<f32>,
    // },
    // TEXT {
    //     #[serde(flatten)]
    //     vector: VectorNode,
    //     characters: String,
    //     style: TypeStyle,
    //     characterStyleOverrides: Vec<f32>,
    // },
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
            // Node::VECTOR(VectorNode { node, .. }) => node,
            // Node::BOOLEAN_OPERATION { vector: VectorNode { node, .. }, .. } => node,
            // Node::STAR(VectorNode { node, .. }) => node,
            // Node::LINE(VectorNode { node, .. }) => node,
            // Node::ELLIPSE(VectorNode { node, .. }) => node,
            // Node::REGULAR_POLYGON(VectorNode { node, .. }) => node,
            // Node::RECTANGLE { vector: VectorNode { node, .. }, .. } => node,
            // Node::TEXT { vector: VectorNode { node, .. }, .. } => node,
            Node::SLICE { node, .. } => node,
            Node::COMPONENT(Frame { node, .. }) => node,
            Node::INSTANCE { frame: Frame { node, .. }, .. } => node,
        }
    }
}