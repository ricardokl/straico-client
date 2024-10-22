use serde::Deserialize;

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
