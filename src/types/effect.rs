use serde::{Deserialize, Serialize};

use super::{blend_mode::BlendMode, colour::Colour, vector::Vector};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EffectType {
    InnerShadow,
    DropShadow,
    LayerBlur,
    BackgroundBlur,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(rename = "type")]
    pub effect_type: EffectType,
    pub visible: bool,
    pub radius: f32,
    #[serde(default)]
    pub color: Colour,
    #[serde(default)]
    pub blend_mode: BlendMode,
    #[serde(default)]
    pub offset: Vector,
    #[serde(default)]
    pub spread: f32,
}
