use crate::config::{ImageDimensions, ImageEffects};
use rand::Rng;

pub fn build_image_url(dimensions: &ImageDimensions, effects: &ImageEffects) -> String {
    let mut url = format!(
        "https://picsum.photos/{}/{}",
        dimensions.width, dimensions.height
    );
    let mut params = Vec::new();

    if effects.grayscale {
        params.push("grayscale".to_string());
    }

    if let Some(blur) = effects.blur {
        params.push(format!("blur={}", blur));
    }

    if let Some(quality) = effects.quality {
        params.push(format!("quality={}", quality));
    }

    let random_id: u32 = rand::rng().random();
    params.push(format!("random={}", random_id));

    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }

    url
}

pub fn generate_filename(index: u32, prefix: &str, format: &str, image_id: Option<&str>) -> String {
    match image_id {
        Some(id) => format!("{}_{}.{}", prefix, id, format),
        None => format!("{}_{:04}.{}", prefix, index + 1, format),
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

