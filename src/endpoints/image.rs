use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Price {
    pub price_per_image: u16,
    pub quantity_images: u8,
    pub total: u16,
}

#[derive(Deserialize)]
pub struct ImageData {
    pub zip: String,
    pub images: Vec<String>,
    pub price: Price,
}

#[derive(Debug, Serialize)]
pub struct ImageRequest {
    pub model: String,
    pub description: String,
    pub size: String,
    pub variations: u8,
}
