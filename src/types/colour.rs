use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
pub struct Colour {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn is_transparent(&self) -> bool {
        self.a == 0.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColourStop {
    pub position: f32,
    pub color: Colour,
}