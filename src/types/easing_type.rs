use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasingType {
    EaseIn,
    EaseOut,
    EaseInAndOut,
    Linear,
}