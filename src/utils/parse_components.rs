use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    types::{
        component::{Component, ComponentSet},
        file::FigmaData,
        frame::Frame,
        node::Node,
        styles::Style,
        token::Token,
    },
    utils::token_values,
};

use askama::Template;
use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
};

#[derive(Template)]
#[template(path = "css.html")]
struct CssTemplate<'a> {
    classes: &'a String,
    rules: &'a Vec<(&'a String, &'a String)>,
}

#[derive(Template, Debug, PartialEq, Eq, Hash)]
#[template(path = "markup.html", escape = "none")]
struct MarkupTemplate {
    tag: String,
    classes: String,
    characters: String,
    children: Vec<MarkupTemplate>,
}

#[derive(Template, Debug)]
#[template(path = "theme.html", escape = "none")]
struct ThemeTemplate {
    rules: BTreeMap<String, Vec<String>>,
}

#[derive(Template, Debug)]
#[template(path = "component.html", escape = "none")]
struct ComponentTemplate {
    name: String,
    styles: String,
    markup: String,
    tokens: String,
}

#[derive(Clone)]
struct ComponentData {
    name: String,
    css: String,
    html: String,
    includes: Vec<String>,
}

pub fn parse(file: FigmaData) {
    let components = file.components;
    let component_sets = file.component_sets;
    let pages = file.document.common().children.iter();
    let styles = file.styles;
    let parent_frame = Frame {
        ..Default::default()
    };
    let mut tokens: HashMap<String, Token> = HashMap::new();
    let mut data: BTreeMap<String, ComponentData> = BTreeMap::new();

    for page in pages {
        let nodes = page.common().children.iter();

        // Get tokens before generating components
        for node in nodes.clone() {
            if let Some(_) = node.is_frame() {
                generate_tokens(node, &styles, &mut tokens);
            }
        }

        for node in nodes {
            if let Some(_) = node.is_component_or_set() {
                let mut element: Vec<MarkupTemplate> = Vec::new();
                let mut css: Vec<String> = Vec::new();
                // Grabs the instances id's to get the extra css need for a component
                let mut includes: Vec<String> = Vec::new();

                generate(
                    node,
                    &parent_frame,
                    &String::new(),
                    &mut element,
                    &mut css,
                    &mut includes,
                    false,
                    &components,
                    &component_sets,
                    &tokens,
                );

                includes.dedup(); // remove duplicates

                data.insert(
                    node.common().id.clone(),
                    ComponentData {
                        name: node.common().get_name(),
                        css: create_css(css.join("\n")),
                        html: create_markup(element, node.is_component_set().is_some()),
                        includes,
                    },
                );
            }
        }
    }

    // TODO: to many clones :/
    for (_, dt) in &data {
        let mut css: Vec<String> = Vec::new();

        // This means that the component has instances of other components so we need to add the css of those
        for id in &dt.includes {
            if let Some(values) = data.get(id) {
                css.push(values.css.clone());
            }
        }

        css.push(dt.css.clone());

        let cmp = ComponentTemplate {
            name: dt.name.clone(),
            styles: css.join(""),
            markup: dt.html.clone(),
            tokens: write_tokens(&tokens),
        };

        write_files(dt.name.clone(), cmp.render().unwrap(), "html");
    }
}

fn generate(
    node: &Node,
    parent_frame: &Frame,
    parent_classes: &String,
    element: &mut Vec<MarkupTemplate>,
    css: &mut Vec<String>,
    includes: &mut Vec<String>,
    is_instance: bool,
    components: &HashMap<String, Component>,
    component_sets: &HashMap<String, ComponentSet>,
    tokens: &HashMap<String, Token>,
) {
    if let Some(frame) = node.is_frame() {
        let classes = format!(
            "{parent_classes}{current_classes}",
            current_classes = frame.get_classes()
        );

        let mut variant_classes = String::new();
        let mut variant_name = frame.node.name.clone();

        if frame.is_variant() {
            variant_classes = parent_frame.get_name();
        }

        // Get correct name and classes for instance, because instance name does not contain variants, so we need to get the info from components
        // and component_sets
        if let Some((_, component_id)) = node.is_instance() {
            if let Some(cmp) = components.get(component_id) {
                variant_name = cmp.get_name();
                let mut id = component_id.to_string();

                if let Some(set) = component_sets.get(&cmp.component_set_id) {
                    id = cmp.component_set_id.to_string();
                    variant_classes = if set.name.eq(&frame.node.name) {
                        frame.node.get_name()
                    } else {
                        format!("{} {}", set.get_name(), frame.node.get_name())
                    }
                }

                includes.push(id);
            }
        }

        // TODO: select the element tag ex: when to use <button></button> instead of <div></div>
        let mut element_markup = MarkupTemplate {
            tag: "div".to_string(),
            classes: frame.get_markup_attributes(variant_classes, variant_name),
            characters: String::new(),
            children: Vec::new(),
        };

        if let None = node.is_component_set() {
            if !is_instance {
                let element_css = frame.css(parent_frame.clone(), tokens);
                css.push(get_styles(
                    &classes,
                    &element_css.iter().collect::<Vec<(&String, &String)>>(),
                ));
            }
        }

        for child in frame.node.children.iter() {
            if let Some((vector, style, characters)) = child.is_text() {
                let text_css = vector.css(style);
                let text_classes = format!("{classes} .{}", vector.get_name());

                if !is_instance {
                    css.push(get_styles(
                        &text_classes,
                        &text_css.iter().collect::<Vec<(&String, &String)>>(),
                    ));
                }
                element_markup.children.push(MarkupTemplate {
                    tag: "span".to_string(),
                    classes: format!(" class=\"{}\"", vector.get_name()),
                    characters: characters.into(),
                    children: Vec::new(),
                });
            } else {
                let condition = match child.is_instance() {
                    Some(_) => true,
                    None => is_instance,
                };
                generate(
                    child,
                    frame,
                    &classes,
                    &mut element_markup.children,
                    css,
                    includes,
                    condition,
                    components,
                    component_sets,
                    tokens,
                );
            }
        }
        element.push(element_markup);
    }
}

fn generate_tokens(
    node: &Node,
    styles: &HashMap<String, Style>,
    tokens: &mut HashMap<String, Token>,
) {
    // TODO: look into removing duplication of logic
    if let Some(frame) = node.is_frame() {
        // "styles": {
        //     "effect": "438:18",
        //     "strokes": "456:20",
        //     "grid": "438:19",
        //     "fills": "438:17",
        //     "text": "436:12",
        // }
        if let Some(style) = &frame.styles {
            for (key, id) in style.iter() {
                if tokens.get(id).is_none() {
                    if let Some(s) = styles.get(id) {
                        let value = match key.as_str() {
                            "fills" => frame.background(),
                            "strokes" => frame.border_colour(),
                            "effect" => frame.box_shadow(None),
                            "grid" => String::new(), // TODO
                            _ => String::new(),
                        };

                        if !value.is_empty() {
                            let (variable, theme) = token_values(s.name.clone());
                            let token = Token {
                                name: s.name.clone(),
                                variable,
                                value,
                                theme,
                            };
                            tokens.insert(id.clone(), token);
                        }
                    }
                }
            }
        }

        // TODO: improve this multiple condition chaining
        for child in frame.node.children.iter() {
            if let Some((vector, _, _)) = child.is_text() {
                if let Some(style) = &vector.styles {
                    for (key, id) in style.iter() {
                        if tokens.get(id).is_none() {
                            if let Some(s) = styles.get(id) {
                                let value = match key.as_str() {
                                    "text" => String::new(), // TODO: this one needs to generate multiple css variables and maybe a class to add to component css
                                    "fills" => vector.text_colour(),
                                    "strokes" => vector.border_colour(),
                                    "effect" => vector.box_shadow(),
                                    _ => String::new(),
                                };

                                if !value.is_empty() {
                                    let (variable, theme) = token_values(s.name.clone());
                                    let token = Token {
                                        name: s.name.clone(),
                                        variable,
                                        value,
                                        theme,
                                    };
                                    tokens.insert(id.clone(), token);
                                }
                            }
                        }
                    }
                }
            }
            generate_tokens(child, styles, tokens);
        }
    }
}

fn get_styles(classes: &String, rules: &Vec<(&String, &String)>) -> String {
    let css_template = CssTemplate { classes, rules };

    css_template.render().unwrap()
}

fn create_markup(values: Vec<MarkupTemplate>, is_set: bool) -> String {
    let mut content = values[0].render().unwrap();

    if is_set {
        // content = format!("{}\n", values[0].children[0].render().unwrap());
        content = String::new();
        let mut seen = HashSet::new();

        // Remove duplicate markup
        for child in &values[0].children {
            if !seen.contains(&child) {
                seen.insert(child);
                content.push_str(&format!("{}\n", child.render().unwrap()));
            }
        }
    }

    content
}

fn create_css(content: String) -> String {
    // TODO: check options to improve result
    // TODO: NOT WORKING AS EXPECTED???? if style rule have attribute selectors lightningcss does not work :(
    // Parse a style sheet from a string.
    let mut stylesheet = StyleSheet::parse(content.as_str(), ParserOptions::default()).unwrap();

    // Minify the stylesheet.
    stylesheet.minify(MinifyOptions::default()).unwrap();

    // Serialize it to a string.
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();

    res.code
}

fn write_tokens(tokens: &HashMap<String, Token>) -> String {
    let mut tk: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (_, token) in tokens.iter() {
        if let None = tk.get(&token.theme) {
            let mut rules: Vec<String> = tokens
                .into_iter()
                .filter(|(_, t)| t.theme.eq(&token.theme))
                .map(|(_id, tok)| format!("{}: {};", tok.variable, tok.value))
                .collect();
            rules.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            tk.insert(token.theme.clone(), rules);
        }
    }

    let tmp = ThemeTemplate { rules: tk.clone() };

    let content = tmp.render().unwrap();

    if !content.is_empty() {
        let _ = std::fs::create_dir_all(format!("figma_output/css"));

        let _ = std::fs::write(format!("figma_output/css/theme.css"), format!("{content}"));
    }

    content
}

fn write_files(name: String, content: String, file_type: &str) {
    if !content.is_empty() {
        let _ = std::fs::create_dir_all(format!("figma_output/components/{name}"));

        let _ = std::fs::write(
            format!("figma_output/components/{name}/{name}.{file_type}"),
            format!("{content}"),
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
    fn test_write_files() {
        // Create a temporary directory
        let dir = tempfile::tempdir().unwrap();

        // Set the current working directory to the temporary directory
        std::env::set_current_dir(dir.path()).unwrap();

        // Define the name and styles
        let name = "test".to_string();
        let styles = ".test { color: red; }".to_string();

        // Call the function
        write_files(name.clone(), styles.clone(), "css");

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
}
