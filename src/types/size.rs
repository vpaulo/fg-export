use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}