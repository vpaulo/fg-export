use std::collections::HashMap;

use crate::prelude::*;
use clap::Parser;
use types::file::FigmaData;

mod cli;
mod error;
mod prelude;

mod types;
mod utils;

async fn load(cmd: &cli::Cli) -> Result<FigmaData> {
    // file: 4TY92OjhtdVceoYBqlkfhU
    // token: figd_J2gB6y3zB10jKf90oBDVLgQ87KFADthb-efzgWkr

    let document = reqwest::Client::new()
        .get(&format!("https://api.figma.com/v1/files/{}", cmd.file))
        .header("X-Figma-Token", cmd.token.clone())
        .send()
        .await?
        .bytes()
        .await?;

    std::fs::create_dir_all("figma_output/")?;
    std::fs::write("figma_output/original_output.json", &document)?;

    let data: FigmaData = serde_json::from_slice(&document)?;
    std::fs::write(
        "figma_output/cache.json",
        serde_json::to_string_pretty(&data).unwrap(),
    )?;
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cli::Cli::parse();

    let file = if !cmd.cache {
        load(&cmd).await?
    } else {
        let full_doc = std::fs::read("figma_output/cache.json")?;
        serde_json::from_slice(&full_doc)?
    };

    if file.components.is_empty() {
        return Err(error::Error::NoComponent);
    }

    // TODO: this is test on typing with the new keyboard, it will take a while to get used to
    // TODO: start parsing component styles and structure
    // TODO: Create separate files for both generated CSS and HTMl
    // TODO: Then create webcomponents from those generated files

    // println!("body = {:?}", serde_json::to_string_pretty(&file).unwrap());
    // print!(">>> {:?}", file.document.common().children.get(0));
    // print!(">>> {:?}", serde_json::to_string_pretty(&file.document.common().children.get(0)).unwrap());
    // TODO: maybe add page filter to the cli??
    // TODO: maybe add option to choose between pixels or rems
    let pages = file.document.common().children.iter();
    for page in pages {
        let components = page
            .common()
            .children
            .iter()
            .filter_map(|node| node.is_component());
        for component in components {
            let mut styles = HashMap::new();
            println!(">>> name: {:?}", component.node.name);
            println!(">>> kebab: {:?}", component.get_name());

            // TODO: check rotation, the sizes change while rotating only n * 90 works
            // (rotation / PI) * 180 OR round(rotation / PI) * 180

            // TODO: Auto layout messes the widths heights
            if component.layout_mode.is_none() {
                if !component.width().is_empty() {
                    styles.insert("width".to_string(), component.width());
                }
                if !component.height().is_empty() {
                    styles.insert("height".to_string(), component.height());
                }
            }

            if !component.border_radius().is_empty() {
                styles.insert("border-radius".to_string(), component.border_radius());
            }

            if !component.background().is_empty() {
                styles.insert("background".to_string(), component.background());
            }

            // GENERATE
            println!(">>> styles: {:?}", styles);

            let css_classes = format!(".{}", component.get_name());
            let mut rules = String::new();

            for (key, value) in styles.iter() {
                rules.push_str(format!("{key}: {value};").as_str());
            }

            // TODO: Find/implement better CSS formatter
            println!("{}", format!("{css_classes} {{{rules}}}"));
        }
    }

    Ok(())
}

// https://github.com/letsgetrusty/json_parsing_example/tree/master
// curl -sH 'X-Figma-Token: figd_J2gB6y3zB10jKf90oBDVLgQ87KFADthb-efzgWkr' 'https://api.figma.com/v1/files/4TY92OjhtdVceoYBqlkfhU' | python -m json.tool
