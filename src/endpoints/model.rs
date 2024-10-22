use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub chat: Vec<ChatModel>,
    pub image: Vec<ImageModel>,
}

#[derive(Debug, Deserialize)]
pub struct ChatModel {
    pub name: String,
    pub model: String,
    pub word_limit: u32,
    pub max_output: u16,
    pub pricing: Pricing,
}

#[derive(Debug, Deserialize)]
pub struct Pricing {
    pub coins: u8,
    pub words: u8,
}

#[derive(Debug, Deserialize)]
pub struct ImageModel {
    pub name: String,
    pub model: String,
    pub pricing: ImagePricing,
}

#[derive(Debug, Deserialize)]
pub struct ImagePricing {
    pub square: SizePricing,
    pub landscape: SizePricing,
    pub portrait: SizePricing,
}

#[derive(Debug, Deserialize)]
pub struct SizePricing {
    pub coins: u8,
    pub size: String,
}
