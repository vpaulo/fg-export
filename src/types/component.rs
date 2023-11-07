use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub key: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub component_set_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSet {
    pub key: String,
    pub name: String,
    pub description: String,
}
