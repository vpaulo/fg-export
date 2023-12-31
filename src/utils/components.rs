use crate::types::{
    frame::Frame, styles::TypeStyle, text::TextTruncation, vector_common::VectorCommon,
};
use std::collections::HashMap;

use askama::Template;

#[derive(Template)]
#[template(path = "css.html")]
struct CssTemplate<'a> {
    classes: &'a String,
    rules: &'a Vec<(&'a String, &'a String)>,
}

pub fn generate(component: &Frame, is_component_set: bool) {
    let mut styles: Vec<String> = Vec::new();

    let parent_frame = Frame {
        ..Default::default()
    };

    // GENERATE CSS
    styles.push(css(
        component.clone(),
        parent_frame,
        String::new(),
        is_component_set,
    ));

    let _ = std::fs::create_dir_all(format!(
        "figma_output/components/{name}",
        name = component.get_name()
    ));

    let _ = std::fs::write(
        format!(
            "figma_output/components/{name}/{name}.css",
            name = component.get_name()
        ),
        format!("{}", styles.join("\n")),
    );

    println!("all: {}", format!("{}", styles.join("\n")));
}

fn css(frame: Frame, parent: Frame, classes: String, is_component_set: bool) -> String {
    let mut styles: Vec<String> = Vec::new();

    println!(">>> name: {:?}", frame.node.name);
    println!(">>> kebab: {:?}", frame.get_name());

    let parent_classes = if !classes.is_empty() {
        format!("{} ", classes)
    } else {
        String::new()
    };

    // TODO: get variant names and values to create attribute styles and pseudo-classes
    let name = frame.get_name();

    if name.contains("=") {
        let variants = name.split(",").filter(|x| !x.to_lowercase().ends_with("default"));
        variants.map(|x| x.split_once("=")).for_each(|x| println!("hey ::: {:?}", x))
    }

    let css_classes = format!("{parent_classes}.{}", frame.get_name());

    // Skip creating rules for component set because what we want is the children styles
    let frame_css = if is_component_set {
        HashMap::new()
    } else {
        frame.css(parent.clone())
    };
    
    let mut sorted: Vec<_> = frame_css.iter().collect();
    sorted.sort_by_key(|a| a.0);

    let css_template = CssTemplate {
        classes: &css_classes.clone(),
        rules: &sorted,
    };
    // TODO: remove this later
    println!("{}", css_template.render().unwrap());

    if !is_component_set {
        styles.push(css_template.render().unwrap());
    }

    let children = frame.node.children.iter();
    for child in children {
        if child.is_frame().is_some() {
            let child_frame = child.is_frame().unwrap();
            let is_child_component_set = child.is_component_set().is_some();
            styles.push(css(
                child_frame.clone(),
                frame.clone(),
                css_classes.clone(),
                is_child_component_set,
            ));
        } else if child.is_text().is_some() {
            let (vector, style) = child.is_text().unwrap();
            styles.push(text_css(vector, style, css_classes.clone()));
        }
    }

    styles.join("\n")
}

fn text_css(vector: &VectorCommon, style: &TypeStyle, classes: String) -> String {
    let mut rules: HashMap<String, String> = HashMap::new();

    let parent_classes = if !classes.is_empty() {
        format!("{} ", classes)
    } else {
        String::new()
    };

    let css_classes = format!("{parent_classes}.{}", vector.get_name());

    if !vector.text_colour().is_empty() {
        rules.insert("color".to_string(), vector.text_colour());
    }

    if !style.font_family.is_empty() {
        rules.insert("font-family".to_string(), style.font_family.to_string());
    }

    if style.font_size != 0.0 {
        rules.insert("font-size".to_string(), format!("{:.0}px", style.font_size));
    }

    if style.font_weight != 0.0 {
        rules.insert(
            "font-weight".to_string(),
            format!("{:.0}", style.font_weight),
        );
    }

    if style.line_height() > 0.0 {
        rules.insert(
            "line-height".to_string(),
            format!("{}", style.line_height()),
        );
    }

    if style.letter_spacing != 0.0 {
        rules.insert(
            "letter-spacing".to_string(),
            format!("{:.0}px", style.letter_spacing),
        );
    }

    if !vector.sizes().is_empty() {
        for (key, value) in vector.sizes().iter() {
            rules.insert(key.to_string(), value.to_string());
        }
    }

    if !style.text_align().is_empty() {
        rules.insert("text-align".to_string(), format!("{}", style.text_align()));
    }

    if !style.text_decoration().is_empty() {
        rules.insert(
            "text-decoration-line".to_string(),
            format!("{}", style.text_decoration()),
        );
    }

    if !style.text_transform().is_empty() {
        rules.insert(
            "text-transform".to_string(),
            format!("{}", style.text_transform()),
        );
    }

    if !style.font_variant().is_empty() {
        rules.insert(
            "font-variant".to_string(),
            format!("{}", style.font_variant()),
        );
    }

    if style.text_truncation == TextTruncation::Ending {
        rules.insert("text-overflow".to_string(), "ellipsis".to_string());

        if let Some(max) = style.max_lines {
            rules.insert("-webkit-box-orient".to_string(), "vertical".to_string());
            rules.insert("-webkit-line-clamp".to_string(), format!("{:.0}", max));
        }
    }

    let mut sorted: Vec<_> = rules.iter().collect();
    sorted.sort_by_key(|a| a.0);

    let css_template = CssTemplate {
        classes: &css_classes,
        rules: &sorted,
    };
    // TODO: remove later
    println!("text styles: {}", css_template.render().unwrap());

    css_template.render().unwrap()
}
