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

    pub fn rgba(&self) -> String {
        let red = self.r * 255.0;
        let green = self.g * 255.0;
        let blue = self.b * 255.0;
        let alpha = self.a;
        return format!("rgba({red},{green},{blue},{alpha})");
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColourStop {
    pub position: f32,
    pub color: Colour,
}
