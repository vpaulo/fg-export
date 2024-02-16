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

#[derive(Template, Debug)]
#[template(path = "markup.html", escape = "none")]
struct MarkupTemplate {
    tag: String,
    classes: String,
    children: Vec<MarkupTemplate>,
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
                let mut element: Vec<MarkupTemplate> = Vec::new();
                let mut styles: Vec<String> = Vec::new();
                generate(
                    node,
                    &parent_frame,
                    &String::new(),
                    &mut element,
                    &mut styles,
                    &components,
                    &component_sets,
                );

                write_styles(node.common().get_name(), styles.join("\n"));
                create_markup(node.common().get_name(), element);
            }
        }
    }
}

fn generate(
    node: &Node,
    parent_frame: &Frame,
    parent_classes: &String,
    element: &mut Vec<MarkupTemplate>,
    styles: &mut Vec<String>,
    components: &HashMap<String, Component>, // TODO: may not be needed, it may be needed for generating HTML
    component_sets: &HashMap<String, ComponentSet>, // TODO: may not be needed, it may be needed for generating HTML
) {
    if let Some(frame) = node.is_frame() {
        let classes = format!(
            "{parent_classes}{current_classes}",
            current_classes = frame.get_classes()
        );

        // TODO: deal with component variants
        let mut element_markup = MarkupTemplate {
            tag: "div".to_string(),
            classes: frame.get_name(),
            children: Vec::new(),
        };

        if let Some(_) = node.is_component_set() {
            // println!(">> SKIP CSS");
        } else {
            let css = frame.css(parent_frame.clone());
            styles.push(get_styles(
                &classes,
                &css.iter().collect::<Vec<(&String, &String)>>(),
            ));
        }

        // if frame.is_variant() {
        //     println!(">> test: {:?} = {}", element.last(), frame.get_name());

        //     element.push(ElementMarkup {
        //         tag: "div".to_string(),
        //         classes: parent_frame.get_name(),
        //     });
        // } else {
            
        // }

        for (index, child) in frame.node.children.iter().enumerate() {
            println!("index: {}", index);
            if let Some((vector, style)) = child.is_text() {
                let text_css = vector.css(style);
                let text_classes = format!("{classes} .{}", vector.get_name());

                styles.push(get_styles(
                    &text_classes,
                    &text_css.iter().collect::<Vec<(&String, &String)>>(),
                ));
                element_markup.children.push(MarkupTemplate {
                    tag: "span".to_string(),
                    classes: vector.get_name(),
                    children: Vec::new(),
                });
            } else if let Some(_) = child.is_instance() {
                // println!(">> SKIP CSS");
            } else {
                generate(
                    child,
                    frame,
                    &classes,
                    &mut element_markup.children,
                    styles,
                    components,
                    component_sets,
                );
            }
        }
        element.push(element_markup);
    }
}

fn get_styles(classes: &String, rules: &Vec<(&String, &String)>) -> String {
    let css_template = CssTemplate { classes, rules };

    css_template.render().unwrap()
}

fn create_markup(name: String, values: Vec<MarkupTemplate>) {
    println!(">>> values: {:?}", values);

    write_files(name, values[0].render().unwrap());
}

fn write_styles(name: String, styles: String) {
    if !styles.is_empty() {
        let _ = std::fs::create_dir_all(format!("figma_output/components/{name}"));

        let _ = std::fs::write(
            format!("figma_output/components/{name}/{name}.css"),
            format!("{styles}"),
        );
    }
}

fn write_files(name: String, markup: String) {
    if !markup.is_empty() {
        let _ = std::fs::create_dir_all(format!("figma_output/components/{name}"));

        let _ = std::fs::write(
            format!("figma_output/components/{name}/{name}.html"),
            format!("{markup}"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use tempfile;

    #[test]
    fn test_write_styles() {
        // Create a temporary directory
        let dir = tempfile::tempdir().unwrap();

        // Set the current working directory to the temporary directory
        std::env::set_current_dir(dir.path()).unwrap();

        // Define the name and styles
        let name = "test".to_string();
        let styles = ".test { color: red; }".to_string();

        // Call the function
        write_styles(name.clone(), styles.clone());

        // Check if the styles were written to the correct file
        let path = format!("figma_output/components/{name}/{name}.css");
        assert!(fs::metadata(&path).is_ok());

        // Read the contents of the file
        let mut file = std::fs::File::open(&path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        // Convert the contents to a string and check if they match the styles
        let contents_str = String::from_utf8(contents).unwrap();
        assert_eq!(contents_str, styles);
    }

    #[test]
    fn test_write_files() {
        // Create a temporary directory
        let dir = tempfile::tempdir().unwrap();

        // Set the current working directory to the temporary directory
        std::env::set_current_dir(dir.path()).unwrap();

        // Define the name and styles
        let name = "test".to_string();
        let markup = "<div class=\"test\"></div>".to_string();

        // Call the function
        write_files(name.clone(), markup.clone());

        // Check if the styles were written to the correct file
        let path = format!("figma_output/components/{name}/{name}.html");
        assert!(fs::metadata(&path).is_ok());

        // Read the contents of the file
        let mut file = std::fs::File::open(&path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        // Convert the contents to a string and check if they match the styles
        let contents_str = String::from_utf8(contents).unwrap();
        assert_eq!(contents_str, markup);
    }
}
