use crate::types::frame::Frame;
use std::collections::HashMap;

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
        }
    }

    let _ = std::fs::write(
        format!("figma_output/css/{}.css", component.get_name()),
        format!("{}", styles.join("\n")),
    );

    println!("all: {}", format!("{}", styles.join("\n")));
}

fn css(frame: Frame, parent: Frame) -> String {
    let mut styles = HashMap::new();
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

    // TODO: Auto layout messes the widths heights
    if frame.layout_mode.is_none() {
        // TODO: when the parent is auto-layout there's some specific cases
        if !frame.width().is_empty() {
            styles.insert("width".to_string(), frame.width());
        }
        if !frame.height().is_empty() {
            styles.insert("height".to_string(), frame.height());
        }

        if parent.layout_mode.is_auto_layout() {
            styles.insert("flex-shrink".to_string(), "0".to_string());
        }
    } else if frame.layout_mode.is_auto_layout() {
        // TODO: should we do inline-flex??
        if frame.node.visible {
            styles.insert("display".to_string(), "flex".to_string());
        }

        if !frame.sizes().is_empty() {
            for (key, value) in frame.sizes().iter() {
                styles.insert(key.to_string(), value.to_string());
            }
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

    // Rotation only works well for 90 * n degrees, for other values like 45deg figma changesn the sizes of width and height.
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
    let mut rules = String::new();

    for (key, value) in styles.iter() {
        rules.push_str(format!("{key}: {value};").as_str());
    }

    println!("{}", format!("{css_classes} {{{rules}}}"));

    format!("{css_classes} {{{rules}}}")
}
