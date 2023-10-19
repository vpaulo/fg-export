use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::{node::Node, component::{ComponentSet, Component}, styles::Style};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaData {
    name: String,
    last_modified: String,
    version: String,
    pub document: Node,
    pub component_sets: HashMap<String, ComponentSet>,
    pub components: HashMap<String, Component>,
    pub styles: HashMap<String, Style>,
}