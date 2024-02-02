use crate::prelude::*;
use clap::Parser;
use types::file::FigmaData;
use utils::parse_components::parse;

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

    // Make sure output folder exists
    std::fs::create_dir_all("figma_output/components")?;

    parse(file);

    Ok(())
}
