use serde::{Deserialize, Serialize};

use super::node_common::NodeCommon;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    #[serde(flatten)]
    pub node: NodeCommon,
    // NOT adding properties for now as it is not needed
}
