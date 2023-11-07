use serde::{Deserialize, Serialize};

use crate::utils::{default_opacity, default_visible};

use super::{colour::Colour, gradient::Gradient};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaintData {
    Solid {
        color: Colour,
    },
    GradientLinear {
        #[serde(flatten)]
        gradient: Gradient,
    },
    GradientRadial {
        #[serde(flatten)]
        gradient: Gradient,
    },
    GradientAngular {
        #[serde(flatten)]
        gradient: Gradient,
    },
    GradientDiamond {
        #[serde(flatten)]
        gradient: Gradient,
    },
    // Image {
    //     #[serde(rename = "scaleMode")]
    //     scale_mode: ScaleMode,
    //     #[serde(rename = "imageTransform", default)]
    //     image_transform: Option<Transform>, // only if scale_mode is STRETCH
    //     #[serde(rename = "scalingFactor", default)]
    //     scaling_factor: Option<f32>, // only if scale_mode is TILE
    //     #[serde(default)] // not present?
    //     rotation: f32,
    //     #[serde(rename = "imageRef")]
    //     image_ref: Option<String>, // sometimes this appears in the character type mapping table and is null
    //     #[serde(rename = "gifRef", default)]
    //     gif_ref: Option<String>,
    //     filters: Option<ImageFilters>,
    // },
    // Emoji,
}

impl PaintData {
    // TODO: maybe pass a parameter into get_solid to choose between RGBA,HEX,HSL??
    pub fn get_solid(&self) -> Option<&Colour> {
        match self {
            PaintData::Solid { color } => Some(color),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Paint {
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    // pub bound_variables: HashMap<string, VariableAlias | Vec<VariableAlias>>
    #[serde(flatten)]
    pub data: PaintData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaintOverride {
    pub fills: Vec<Paint>,
    pub inherit_fil_style_id: String,
}
