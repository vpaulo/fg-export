use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub path: String,
    pub winding_rule: String,
}
