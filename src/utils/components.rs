use crate::types::{
    frame::Frame, styles::TypeStyle, text::TextTruncation, vector_common::VectorCommon,
};
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
    let mut styles: Vec<String> = Vec::new();
    let mut rules: HashMap<String, String> = HashMap::new();

    println!(">>> name: {:?}", frame.node.name);
    println!(">>> kebab: {:?}", frame.get_name());

    let parent_classes = if !parent.get_name().is_empty() {
        format!(".{} ", parent.get_name())
    } else {
        String::new()
    };

    if !frame.node.visible {
        rules.insert("display".to_string(), "none".to_string());
    }

    if frame.clips_content {
        rules.insert("overflow".to_string(), "hidden".to_string());
    }

    if !frame.sizes(parent.clone()).is_empty() {
        for (key, value) in frame.sizes(parent.clone()).iter() {
            rules.insert(key.to_string(), value.to_string());
        }
    }

    if frame.layout_mode.is_auto_layout() {
        if frame.node.visible {
            rules.insert("display".to_string(), "flex".to_string());
        }

        if !frame.layout_wrap().is_empty() {
            rules.insert("flex-wrap".to_string(), frame.layout_wrap());
        }

        if frame.layout_mode.is_vertical() {
            rules.insert("flex-direction".to_string(), "column".to_string());
        }

        if !frame.alignment().is_empty() {
            for (key, value) in frame.alignment().iter() {
                rules.insert(key.to_string(), value.to_string());
            }
        }

        if !frame.gap().is_empty() {
            rules.insert("gap".to_string(), frame.gap());
        }

        if !frame.padding().is_empty() {
            rules.insert("padding".to_string(), frame.padding());
        }
    }

    // Rotation only works well for 90 * n degrees, for other values like 45deg figma changes the sizes of width and height.
    if !frame.rotation().is_empty() {
        rules.insert("transform".to_string(), frame.rotation());
    }

    if !frame.border_radius().is_empty() {
        rules.insert("border-radius".to_string(), frame.border_radius());
    }

    if !frame.border().is_empty() {
        for (key, value) in frame.border().iter() {
            rules.insert(key.to_string(), value.to_string());
        }
    }

    if !frame.background().is_empty() {
        rules.insert("background".to_string(), frame.background());
    }

    if !frame.box_shadow().is_empty() {
        rules.insert("box-shadow".to_string(), frame.box_shadow());
    }

    if !frame.blur().is_empty() {
        rules.insert("filter".to_string(), frame.blur());
    }

    if !frame.background_blur().is_empty() {
        rules.insert("backdrop-filter".to_string(), frame.background_blur());
    }

    let css_classes = format!("{parent_classes}.{}", frame.get_name());

    let css_template = CssTemplate {
        classes: &css_classes,
        rules: &rules,
    };
    // TODO: remove this later
    println!("{}", css_template.render().unwrap());

    styles.push(css_template.render().unwrap());

    let children = frame.node.children.iter();
    for child in children {
        if child.is_frame().is_some() {
            let child_frame = child.is_frame().unwrap();
            styles.push(css(child_frame.clone(), frame.clone()));
        } else if child.is_text().is_some() {
            let (vector, style) = child.is_text().unwrap();
            styles.push(text_css(vector, style, frame.clone()));
        }
    }

    styles.join("\n")
}

fn text_css(vector: &VectorCommon, style: &TypeStyle, parent: Frame) -> String {
    let mut rules: HashMap<String, String> = HashMap::new();

    let parent_classes = if !parent.get_name().is_empty() {
        format!(".{} ", parent.get_name())
    } else {
        String::new()
    };

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

    // textAutoResize will tell how to set the sizes
    // ideally run frame.sizes on this or add the missing keys if VectorCommon???

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

    let css_classes = format!("{parent_classes}.{}", vector.get_name());

    let css_template = CssTemplate {
        classes: &css_classes,
        rules: &rules,
    };
    // TODO: remove later
    println!("text styles: {}", css_template.render().unwrap());

    css_template.render().unwrap()
}
