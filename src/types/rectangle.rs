use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}
// impl Rectangle {
//     pub fn x(&self) -> f32 {
//         self.x.unwrap_or(0.0)
//     }
//     pub fn y(&self) -> f32 {
//         self.y.unwrap_or(0.0)
//     }
//     pub fn width(&self) -> f32 {
//         self.width.unwrap_or(0.0)
//     }
//     pub fn height(&self) -> f32 {
//         self.height.unwrap_or(0.0)
//     }
// }
