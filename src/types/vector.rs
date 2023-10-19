use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Vector {
    pub x: Option<f32>,
    pub y: Option<f32>,
}
impl Vector {
    pub fn is_valid(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }
    pub fn x(&self) -> f32 {
        self.x.unwrap_or(0.0)
    }
    pub fn y(&self) -> f32 {
        self.y.unwrap_or(0.0)
    }
}