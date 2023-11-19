use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct StrokeWeights {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
