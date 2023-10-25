use crate::prelude::*;
use clap::Parser;
use types::file::FigmaData;

mod error;
mod prelude;
mod cli;

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
    std::fs::write("figma_output/cache.json", serde_json::to_string_pretty(&data).unwrap())?;
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
    Ok(())
}

// https://github.com/letsgetrusty/json_parsing_example/tree/master
// curl -sH 'X-Figma-Token: figd_J2gB6y3zB10jKf90oBDVLgQ87KFADthb-efzgWkr' 'https://api.figma.com/v1/files/4TY92OjhtdVceoYBqlkfhU' | python -m json.tool
