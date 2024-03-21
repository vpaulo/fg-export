use convert_case::{Case, Casing};

use crate::types::effect::Effect;

pub mod parse_components;

pub fn default_visible() -> bool {
    true
}

pub fn default_opacity() -> f32 {
    1.0
}

pub fn default_effects() -> Vec<Effect> {
    Vec::new()
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

pub fn parse_name(name: &String) -> String {
    name.replace("/", " ")
        .replace("•", " ")
        .replace(".", " ")
        .replace(":", " ")
        .replace("\u{a0}", " ")
        .to_case(Case::Kebab)
}

pub fn token_values(name: String) -> (String, String) {
    let mut variable = format!("--{}", parse_name(&name));
    let mut theme = String::from(":root");

    if name.contains("/") {
        let list = name.split("/").collect::<Vec<&str>>();
        if list[0].contains("theme") {
            theme = parse_name(&list[0].to_string());
            variable = format!("--{}", parse_name(&list[1..].join("-")));
        } else {
            variable = format!("--{}", parse_name(&list.join("-")));
        }
    }

    (variable, theme)
}
