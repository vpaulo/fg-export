use crate::types::effect::Effect;

pub mod components;

pub fn default_visible() -> bool {
    true
}

pub fn default_opacity() -> f32 {
    1.0
}

pub fn default_locked() -> bool {
    false
}
pub fn default_effects() -> Vec<Effect> {
    Vec::new()
}
pub fn default_clips_content() -> bool {
    false
}

pub fn default_line_height_percent() -> f32 {
    100.0
}
pub fn default_paragraph_spacing() -> f32 {
    0.0
}
pub fn default_paragraph_indent() -> f32 {
    0.0
}

pub fn default_list_spacing() -> f32 {
    0.0
}
