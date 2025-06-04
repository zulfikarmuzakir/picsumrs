use crate::cli::{Cli, Commands};
use crate::error::{Error, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub operation: Operation,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Download(DownloadConfig),
    Info(InfoConfig),
    List(ListConfig),
    Search(SearchConfig),
}

#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub count: u32,
    pub dimensions: ImageDimensions,
    pub output_dir: PathBuf,
    pub effects: ImageEffects,
    pub concurrent_limit: usize,
    pub filename_config: FilenameConfig,
}

#[derive(Debug, Clone)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct ImageEffects {
    pub grayscale: bool,
    pub blur: Option<u32>,
    pub quality: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct FilenameConfig {
    pub prefix: String,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct InfoConfig {
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct ListConfig {
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub author: String,
    pub limit: u32,
}

fn validate_download_params(
    count: u32,
    width: u32,
    height: u32,
    blur: Option<u32>,
    quality: Option<u32>,
    concurrent: usize,
) -> Result<()> {
    if count == 0 {
        return Err(Error::Config("Count must be greater than 0".to_string()));
    }

    if width == 0 || height == 0 {
        return Err(Error::Config(
            "Width and height must be greater than 0".to_string(),
        ));
    }

    if let Some(blur_level) = blur {
        if blur_level == 0 || blur_level >= 10 {
            return Err(Error::Config(
                "Blur level must be between 1 and 10".to_string(),
            ));
        }
    }

    if let Some(quality_level) = quality {
        if quality_level == 0 || quality_level > 100 {
            return Err(Error::Config(
                "Quality must be between 1 and 100".to_string(),
            ));
        }
    }

    if concurrent == 0 || concurrent > 20 {
        return Err(Error::Config(
            "Concurrent downloads must be between 1 and 20".to_string(),
        ));
    }

    Ok(())
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self> {
        let operation = match cli.command {
            Commands::Download {
                count,
                width,
                height,
                output,
                grayscale,
                blur,
                quality,
                concurrent,
                prefix,
                format,
            } => {
                self::validate_download_params(count, width, height, blur, quality, concurrent)?;

                Operation::Download(DownloadConfig {
                    count,
                    dimensions: ImageDimensions { width, height },
                    output_dir: PathBuf::from(output),
                    effects: ImageEffects {
                        grayscale,
                        blur,
                        quality,
                    },
                    concurrent_limit: concurrent,
                    filename_config: FilenameConfig { prefix, format },
                })
            }
            Commands::Info { id } => Operation::Info(InfoConfig { id }),
            Commands::List { page, limit } => {
                if limit > 100 {
                    return Err(Error::Config("Limit cannot exceed 100".to_string()));
                }
                Operation::List(ListConfig { page, limit })
            }

            Commands::Search { author, limit } => {
                if author.trim().is_empty() {
                    return Err(Error::Config("Author name cannot be empty".to_string()));
                }

                Operation::Search(SearchConfig { author, limit })
            }
        };

        Ok(Config {
            operation,
            verbose: cli.verbose,
        })
    }
}
