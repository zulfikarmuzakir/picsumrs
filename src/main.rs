use clap::Parser;
use cli::Cli;
use config::Config;
use downloader::Downloader;
use crate::error::Result;

mod cli;
mod config;
mod downloader;
mod error;
mod picsum;
mod progress;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from_cli(cli)?;

    let downloader = Downloader::new(config).await?;
    downloader.execute().await?;

    Ok(())
}
