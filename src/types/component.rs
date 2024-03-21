use serde::{Deserialize, Serialize};

use crate::utils::parse_name;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub key: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub component_set_id: String,
}

impl Component {
    pub fn get_name(&self) -> String {
        parse_name(&self.name)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSet {
    pub key: String,
    pub name: String,
    pub description: String,
}

impl ComponentSet {
    pub fn get_name(&self) -> String {
        parse_name(&self.name)
    }
}
