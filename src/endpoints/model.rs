use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub chat: Vec<ChatModel>,
    // 'image' will be implemented later
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
