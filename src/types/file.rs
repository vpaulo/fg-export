use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    component::{Component, ComponentSet},
    node::Node,
    styles::Style,
};

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
