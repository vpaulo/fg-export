use crate::{prelude::*, utils::components::generate};
use clap::Parser;
use types::file::FigmaData;

mod cli;
mod error;
mod prelude;

mod types;
mod utils;

async fn load(figma_config: &cli::FigmaConfig) -> Result<FigmaData> {
    let document = reqwest::Client::new()
        .get(&format!(
            "https://api.figma.com/v1/files/{}",
            figma_config.file.clone()
        ))
        .header("X-Figma-Token", figma_config.token.clone())
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

    let file = if let Some(figma_config) = cmd.figma_config {
        load(&figma_config).await?
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

    // Make sure output folder exists
    std::fs::create_dir_all("figma_output/css")?;

    let pages = file.document.common().children.iter();
    for page in pages {
        let components = page
            .common()
            .children
            .iter()
            .filter_map(|node| node.is_component());
        for component in components {
            generate(component);

            // TODO: GENERATE HTML
            // TODO: GENERATE WEB COMPONENTS
            // TODO: GENERATE TOKENS
            // TODO: GENERATE DESIGN_TOKENS

            // TODO: Find/implement better CSS formatter
            // println!("{}", format!("{css_classes} {{{rules}}}"));
            // std::fs::write(
            //     format!("figma_output/css/{}.css", component.get_name()),
            //     format!("{css_classes} {{{rules}}}"),
            // )?;
        }
    }

    Ok(())
}
