use crate::types::{frame::Frame, vector_common::VectorCommon, styles::TypeStyle};
use std::collections::HashMap;

use askama::Template;

#[derive(Template)]
#[template(path = "css.html")]
struct CssTemplate<'a> {
    classes: &'a String,
    rules: &'a HashMap<String, String>,
}

pub fn generate(component: &Frame) {
    let mut styles: Vec<String> = Vec::new();

    let parent_frame = Frame {
        ..Default::default()
    };

    // GENERATE CSS
    styles.push(css(component.clone(), parent_frame));

    let children = component.node.children.iter();
    for child in children {
        if child.is_frame().is_some() {
            let frame = child.is_frame().unwrap();
            styles.push(css(frame.clone(), component.clone()));
        } else if child.is_text().is_some() {
            let (vector, style) = child.is_text().unwrap();
            styles.push(text(vector, style, component.clone()));
        }
    }

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

fn css(frame: Frame, parent: Frame) -> String {
    let mut styles: HashMap<String, String> = HashMap::new();
    println!(">>> name: {:?}", frame.node.name);
    println!(">>> kebab: {:?}", frame.get_name());

    let parent_classes = if !parent.get_name().is_empty() {
        format!(".{} ", parent.get_name())
    } else {
        String::new()
    };

    if !frame.node.visible {
        styles.insert("display".to_string(), "none".to_string());
    }

    if frame.clips_content {
        styles.insert("overflow".to_string(), "hidden".to_string());
    }

    if !frame.sizes(parent.clone()).is_empty() {
        for (key, value) in frame.sizes(parent.clone()).iter() {
            styles.insert(key.to_string(), value.to_string());
        }
    }

    if frame.layout_mode.is_auto_layout() {
        if frame.node.visible {
            styles.insert("display".to_string(), "flex".to_string());
        }

        if !frame.layout_wrap().is_empty() {
            styles.insert("flex-wrap".to_string(), frame.layout_wrap());
        }

        if frame.layout_mode.is_vertical() {
            styles.insert("flex-direction".to_string(), "column".to_string());
        }

        if !frame.alignment().is_empty() {
            for (key, value) in frame.alignment().iter() {
                styles.insert(key.to_string(), value.to_string());
            }
        }

        if !frame.gap().is_empty() {
            styles.insert("gap".to_string(), frame.gap());
        }

        if !frame.padding().is_empty() {
            styles.insert("padding".to_string(), frame.padding());
        }
    }

    // Rotation only works well for 90 * n degrees, for other values like 45deg figma changes the sizes of width and height.
    if !frame.rotation().is_empty() {
        styles.insert("transform".to_string(), frame.rotation());
    }

    if !frame.border_radius().is_empty() {
        styles.insert("border-radius".to_string(), frame.border_radius());
    }

    if !frame.border().is_empty() {
        for (key, value) in frame.border().iter() {
            styles.insert(key.to_string(), value.to_string());
        }
    }

    if !frame.background().is_empty() {
        styles.insert("background".to_string(), frame.background());
    }

    if !frame.box_shadow().is_empty() {
        styles.insert("box-shadow".to_string(), frame.box_shadow());
    }

    if !frame.blur().is_empty() {
        styles.insert("filter".to_string(), frame.blur());
    }

    if !frame.background_blur().is_empty() {
        styles.insert("backdrop-filter".to_string(), frame.background_blur());
    }

    let css_classes = format!("{parent_classes}.{}", frame.get_name());

    let css = CssTemplate {
        classes: &css_classes,
        rules: &styles,
    };
    println!("{}", css.render().unwrap());

    css.render().unwrap()
}

fn text(vector: &VectorCommon, style: &TypeStyle, parent: Frame) -> String {
    let mut styles: HashMap<String, String> = HashMap::new();

    let parent_classes = if !parent.get_name().is_empty() {
        format!(".{} ", parent.get_name())
    } else {
        String::new()
    };


    if !vector.text_colour().is_empty() {
        styles.insert("color".to_string(), vector.text_colour());
    }

    if !style.font_family.is_empty() {
        styles.insert("font-family".to_string(), style.font_family.to_string());
    }

    if style.font_size != 0.0 {
        styles.insert("font-size".to_string(), format!("{:.0}px", style.font_size));
    }

    if style.font_weight != 0.0 {
        styles.insert("font-weight".to_string(), format!("{:.0}", style.font_weight));
    }

    if style.line_height() > 0.0 {
        styles.insert("line-height".to_string(), format!("{:.0}", style.line_height()));
    }








    let css_classes = format!("{parent_classes}.{}", vector.get_name());

    let css = CssTemplate {
        classes: &css_classes,
        rules: &styles,
    };
    println!("{}", css.render().unwrap());

    css.render().unwrap()
}
