use serde::{Deserialize, Serialize};

use super::vector::Vector;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FrameOffset {
    pub node_id: String,
    pub node_offset: Vector,
}