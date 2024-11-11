use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ModelData {
    chat: Vec<ChatModel>,
    image: Vec<ImageModel>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ChatModel {
    name: String,
    model: String,
    word_limit: u32,
    max_output: u16,
    pricing: ChatPricing,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ChatPricing {
    coins: u8,
    words: u8,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ImageModel {
    name: String,
    model: String,
    pricing: ImagePricing,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ImagePricing {
    square: SizePricing,
    landscape: SizePricing,
    portrait: SizePricing,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SizePricing {
    coins: u8,
    size: String,
}
