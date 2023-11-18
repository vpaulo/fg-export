use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct FigmaConfig {
    /// Figma access token
    #[arg(short = 't', long = "token")]
    pub token: String,
    /// Figma file
    pub file: String,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional Figma configuration
    #[clap(flatten)]
    pub figma_config: Option<FigmaConfig>,

    /// If set, don't connect to the network, but use the `figma_output/cache.json`
    #[arg(long, conflicts_with_all = &["token", "file"])]
    pub cache: bool,
}
