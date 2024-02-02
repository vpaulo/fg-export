use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

use super::node::Node;
use crate::utils;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct NodeCommon {
    pub id: String,
    pub name: String,
    #[serde(default = "utils::default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub children: Vec<Node>,
}

impl NodeCommon {
    pub fn get_name(&self) -> String {
        self.name.to_case(Case::Kebab)
    }
}
