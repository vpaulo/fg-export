use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConstraintType {
    Scale,
    Width,
    Height,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Constraint {
    #[serde(rename = "type")]
    pub constraint_type: ConstraintType,
    pub value: f32,
}
