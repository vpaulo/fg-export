use crate::prelude::*;
use clap::Parser;

mod error;
mod prelude;
mod cli;

mod types;
mod utils;


async fn load(cmd: &cli::Cli) -> Result<types::file::FigmaData> {
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
    std::fs::write("figma_output/cache.json", &document)?;

    let data: types::file::FigmaData = serde_json::from_slice(&document)?;
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
    
    // println!("body = {:?}", serde_json::to_string_pretty(&res).unwrap());
    println!("body = {:?}", serde_json::to_string_pretty(&file).unwrap());
    Ok(())
}

// https://github.com/letsgetrusty/json_parsing_example/tree/master
// curl -sH 'X-Figma-Token: figd_J2gB6y3zB10jKf90oBDVLgQ87KFADthb-efzgWkr' 'https://api.figma.com/v1/files/4TY92OjhtdVceoYBqlkfhU' | python -m json.tool
