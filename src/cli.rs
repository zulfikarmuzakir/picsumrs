use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "picsumrs")]
#[command(version = "0.1.0")]
#[command(about = "A powerful CLI tool for downloading images from Picsum Photos")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    // enable verbose outout
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Download {
        #[arg(short, long, default_value = "1")]
        count: u32,

        #[arg(short, long, default_value = "1920")]
        width: u32,

        #[arg(short = 'H', long, default_value = "1080")]
        height: u32,

        #[arg(short, long, default_value = "downloads")]
        output: String,

        #[arg(short, long)]
        grayscale: bool,
        
        /// Apply blur effect (1-10)
        #[arg(short, long)]
        blur: Option<u32>,
        
        /// JPEG quality (1-100)
        #[arg(short = 'q', long)]
        quality: Option<u32>,
        
        /// Number of concurrent downloads
        #[arg(short = 'j', long, default_value = "4")]
        concurrent: usize,
        
        /// Custom filename prefix
        #[arg(short = 'p', long, default_value = "picsum")]
        prefix: String, 
        
        /// Image format (jpg, png, webp)
        #[arg(short, long, default_value = "jpg")]
        format: String,
    }
}
