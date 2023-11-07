use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Figma access token
    #[arg(short = 't', long = "token")]
    pub token: String,
    /// If set, don't connect to the network, but use the `figma_output/cache.json`
    #[arg(long)]
    pub cache: bool,
    /// Figma file
    pub file: String,
}
