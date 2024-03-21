use serde::{Deserialize, Serialize};

use super::node::Node;
use crate::utils::{self, parse_name};

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
        parse_name(&self.name.to_string())
    }
}
