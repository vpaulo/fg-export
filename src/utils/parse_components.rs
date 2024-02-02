use std::collections::HashMap;

use crate::types::{
    component::{Component, ComponentSet},
    file::FigmaData,
    frame::Frame,
    node::Node,
};

use askama::Template;

#[derive(Template)]
#[template(path = "css.html")]
struct CssTemplate<'a> {
    classes: &'a String,
    rules: &'a Vec<(&'a String, &'a String)>,
}

pub fn parse(file: FigmaData) {
    let components = file.components;
    let component_sets = file.component_sets;
    let pages = file.document.common().children.iter();
    let parent_frame = Frame {
        ..Default::default()
    };

    for page in pages {
        let nodes = page.common().children.iter();

        for node in nodes {
            if let Some(_) = node.is_component_or_set() {
                let styles = generate(
                    node,
                    &parent_frame,
                    &String::new(),
                    &components,
                    &component_sets,
                );

                write_styles(node.common().get_name(), styles);
            }
        }
    }
}

fn generate(
    node: &Node,
    parent_frame: &Frame,
    parent_classes: &String,
    components: &HashMap<String, Component>, // TODO: may not be needed, it may be needed for generating HTML
    component_sets: &HashMap<String, ComponentSet>, // TODO: may not be needed, it may be needed for generating HTML
) -> String {
    // println!("{:?} -- {} -- {:?}", node.common().name, node.common().id, components.get(&node.common().id));
    let mut styles: Vec<String> = Vec::new();

    if let Some(frame) = node.is_frame() {
        let classes = format!(
            "{parent_classes}{current_classes}",
            current_classes = frame.get_classes()
        );

        if let Some(_) = node.is_component_set() {
            // println!(">> SKIP CSS");
        } else {
            let css = frame.css(parent_frame.clone());
            styles.push(get_styles(
                &classes,
                &css.iter().collect::<Vec<(&String, &String)>>(),
            ));
        }

        let children = frame.node.children.iter();
        for child in children {
            if let Some((vector, style)) = child.is_text() {
                let text_css = vector.css(style);
                let text_classes = format!("{classes} .{}", vector.get_name());

                styles.push(get_styles(
                    &text_classes,
                    &text_css.iter().collect::<Vec<(&String, &String)>>(),
                ));
            } else if let Some(_) = child.is_instance() {
                // println!(">> SKIP CSS");
            } else {
                styles.push(generate(child, frame, &classes, components, component_sets));
            }
        }
    }

    styles.join("\n")
}

fn get_styles(classes: &String, rules: &Vec<(&String, &String)>) -> String {
    let css_template = CssTemplate { classes, rules };

    css_template.render().unwrap()
}

fn write_styles(name: String, styles: String) {
    if !styles.is_empty() {
        // println!("styles: {name} = {styles}");
        let _ = std::fs::create_dir_all(format!("figma_output/components/{name}"));

        let _ = std::fs::write(
            format!("figma_output/components/{name}/{name}.css"),
            format!("{}", styles),
        );
    }
}
