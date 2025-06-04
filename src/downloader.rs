use crate::config::{self, Config, Operation};
use crate::error::Result;
use crate::picsum::PicsumClient;
use crate::progress::{self, ProgressTracker};
use crate::utils::{build_image_url, format_file_size, generate_filename};
use rand::seq::index;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct Downloader {
    config: Config,
    client: PicsumClient,
}

impl Downloader {
    pub async fn new(config: Config) -> Result<Self> {
        let client = PicsumClient::new()?;

        Ok(Self { config, client })
    }

    pub async fn execute(&self) -> Result<()> {
        match &self.config.operation {
            Operation::Download(download_config) => self.handle_download(download_config).await,
            Operation::Info(info_config) => self.handle_info(info_config).await,
            Operation::List(list_config) => self.handle_list(list_config).await,
            Operation::Search(search_config) => self.handle_search(search_config).await,
        }
    }

    async fn handle_download(&self, config: &crate::config::DownloadConfig) -> Result<()> {
        fs::create_dir_all(&config.output_dir)?;

        self.print_download_info(config);

        let progress = ProgressTracker::new(config.count as u64, "Downloading");

        let semaphore = Arc::new(Semaphore::new(config.concurrent_limit));
        let mut handles = Vec::new();
        let mut total_bytes = 0u64;

        for i in 0..config.count {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = self.client.clone();
            let config = config.clone();
            let progress = progress.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;
                let result = Self::download_single_image(&client, &config, i).await;
                progress.inc(1);
                result
            });

            handles.push(handle);
        }

        let mut success_count = 0;
        for handle in handles {
            match handle.await.unwrap() {
                Ok(bytes) => {
                    success_count += 1;
                    total_bytes += bytes;
                }
                Err(e) => {
                    if self.config.verbose {
                        eprintln!("❌ Download failed: {}", e);
                    }
                }
            }
        }

        progress.finish_with_message(&format!(
            "✅ Downloaded {}/{} images ({}) in {:.2}s",
            success_count,
            config.count,
            format_file_size(total_bytes),
            progress.elapsed().as_secs_f32()
        ));

        Ok(())
    }

    async fn handle_info(&self, config: &crate::config::InfoConfig) -> Result<()> {
        println!("🔍 Fetching image information...");

        let info = self.client.get_image_info(config.id).await?;

        println!("\n📷 Image Details:");
        println!("┌─────────────────────────────────────");
        println!("│ ID: {}", info.id);
        println!("│ Author: {}", info.author);
        println!("│ Dimensions: {}×{}", info.width, info.height);
        println!("│ URL: {}", info.url);
        println!("│ Download URL: {}", info.download_url);
        println!("└─────────────────────────────────────");

        Ok(())
    }

    async fn handle_list(&self, config: &crate::config::ListConfig) -> Result<()> {
        println!("📋 Fetching image list (page {})...", config.page);

        let images = self.client.list_images(config.page, config.limit).await?;

        if images.is_empty() {
            println!("No images found on page {}", config.page);
            return Ok(());
        }

        println!("\n📸 Available Images:");
        println!("┌──────┬─────────────────────────────┬─────────────┬──────────────────────────────────────────────────");
        println!("│ ID   │ Author                      │ Size        │ URL");
        println!("├──────┼─────────────────────────────┼─────────────┼──────────────────────────────────────────────────");

        for img in images {
            println!(
                "│ {:4} │ {:27} │ {:4}×{:4}    │ {}",
                img.id,
                truncate_string(&img.author, 27),
                img.width,
                img.height,
                img.url
            );
        }

        println!("└──────┴─────────────────────────────┴─────────────┴──────────────────────────────────────────────────");

        Ok(())
    }

    async fn handle_search(&self, config: &crate::config::SearchConfig) -> Result<()> {
        println!("🔎 Searching for images by '{}'...", config.author);

        let images = self
            .client
            .search_by_author(&config.author, config.limit)
            .await?;

        if images.is_empty() {
            println!("No images found by author '{}'", config.author);
            return Ok(());
        }

        println!("\n🎨 Found {} image(s):", images.len());
        println!("┌──────┬─────────────────────────────┬─────────────┬──────────────────────────────────────────────────");
        println!("│ ID   │ Author                      │ Size        │ URL");
        println!("├──────┼─────────────────────────────┼─────────────┼──────────────────────────────────────────────────");

        for img in images {
            println!(
                "│ {:4} │ {:27} │ {:4}×{:4}    │ {}",
                img.id,
                truncate_string(&img.author, 27),
                img.width,
                img.height,
                img.url
            );
        }

        println!("└──────┴─────────────────────────────┴─────────────┴──────────────────────────────────────────────────");

        Ok(())
    }

    async fn download_single_image(
        client: &PicsumClient,
        config: &crate::config::DownloadConfig,
        index: u32,
    ) -> Result<u64> {
        let url = build_image_url(&config.dimensions, &config.effects);
        let bytes = client.download_image_bytes(&url).await?;

        let image_id = None;
        let filename = generate_filename(
            index,
            &config.filename_config.prefix,
            &config.filename_config.format,
            image_id,
        );

        let filepath = config.output_dir.join(&filename);
        fs::write(&filepath, &bytes)?;

        Ok(bytes.len() as u64)
    }

    fn print_download_info(&self, config: &crate::config::DownloadConfig) {
        println!("🖼️  Picsum Image Downloader v0.1.0");
        println!("┌─────────────────────────────────────");
        println!("│ 📁 Output: {}", config.output_dir.display());
        println!(
            "│ 📏 Resolution: {}×{}",
            config.dimensions.width, config.dimensions.height
        );
        println!("│ 📊 Count: {}", config.count);
        println!("│ 🚀 Concurrent: {}", config.concurrent_limit);

        if config.effects.grayscale {
            println!("│ 🎨 Effect: Grayscale");
        }

        if let Some(blur) = config.effects.blur {
            println!("│ 🌫️  Blur: Level {}", blur);
        }

        if let Some(quality) = config.effects.quality {
            println!("│ 🎯 Quality: {}%", quality);
        }

        println!("└─────────────────────────────────────\n");
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
