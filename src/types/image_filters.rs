use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImageFilters {
    pub exposure: Option<f32>,    //Status: Needs scaling work.
    pub contrast: Option<f32>,    //Status: Supported
    pub saturation: Option<f32>,  //Status: Supported
    pub temperature: Option<f32>, //horiontal axis, blue to amber
    pub tint: Option<f32>,        //vertical axis, green to magenta
    pub highlights: Option<f32>,  //adjust only lighter areas of image
    pub shadows: Option<f32>,     //adjust only darker areas of image
}
