use crate::{
    config::ImageDimensions,
    error::{Error, Result},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub author: String,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub download_url: String,
}

#[derive(Debug, Clone)]
pub struct PicsumClient {
    client: Client,
    base_url: String,
}

impl PicsumClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("Picsum-CLI-Downloader/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: "https://picsum.photos".to_string(),
        })
    }

    pub async fn get_image_info(&self, id: u32) -> Result<ImageInfo> {
        let url = format!("{}/id/{}/info", self.base_url, id);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::Api(format!("Image with ID {} not found", id)));
        }

        let info: ImageInfo = response.json().await?;
        Ok(info)
    }

    pub async fn list_images(&self, page: u32, limit: u32) -> Result<Vec<ImageInfo>> {
        let url = format!("{}/v2/list?page={}&limit={}", self.base_url, page, limit);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(Error::Api("Failed to fetch image list".to_string()));
        }

        let images: Vec<ImageInfo> = response.json().await?;

        Ok(images)
    }

    pub async fn search_by_author(&self, author: &str, limit: u32) -> Result<Vec<ImageInfo>> {
        let mut all_images = Vec::new();
        let mut page = 1;
        let page_limit = 30;

        while all_images.len() < limit as usize && page <= 10 {
            let images = self.list_images(page, page_limit).await?;
            if images.is_empty() {
                break;
            }

            let matching_images: Vec<ImageInfo> = images
                .into_iter()
                .filter(|img| img.author.to_lowercase().contains(&author.to_lowercase()))
                .collect();

            all_images.extend(matching_images);
            page += 1;
        }

        all_images.truncate(limit as usize);
        Ok(all_images)
    }

    pub async fn download_image_bytes(&self, url: &str) -> Result<bytes::Bytes> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(Error::Api(format!(
                "Failed to download image: HTTP {}",
                response.status()
            )));
        }

        let bytes = response.bytes().await?;
        Ok(bytes)
    }
}
