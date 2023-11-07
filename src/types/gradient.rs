use serde::{Deserialize, Serialize};

use super::{blend_mode::BlendMode, colour::Colour, vector::Vector};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Gradient {
    #[serde(default, rename = "blendMode")]
    pub blend_mode: BlendMode,
    #[serde(rename = "gradientHandlePositions")]
    pub gradient_handle_positions: Vec<Vector>,
    #[serde(rename = "gradientStops")]
    pub gradient_stops: Vec<ColorStop>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColorStop {
    pub position: f32,
    pub color: Colour,
}
