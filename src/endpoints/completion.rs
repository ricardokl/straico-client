use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CompletionsData {
    pub completions: HashMap<String, Model>,
    pub overall_price: Price,
    pub overall_words: Words,
}

#[derive(Deserialize)]
pub struct Price {
    pub input: f32,
    pub output: f32,
    pub total: f32,
}

#[derive(Deserialize)]
pub struct Words {
    pub input: u32,
    pub output: u32,
    pub total: u32,
}

#[derive(Deserialize)]
pub struct Model {
    pub completion: Completion,
    pub price: Price,
    pub words: Words,
}

#[derive(Deserialize)]
pub struct Completion {
    pub choices: Vec<Choice>,
    pub id: String,
    pub model: String,
    pub created: usize,
    pub usage: Usage,
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_token: u32,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
    pub index: u8,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CompletionRequest {
    pub models: Vec<String>,
    pub file_urls: Vec<String>,
    pub youtube_urls: Vec<String>,
    pub message: String,
    pub display_transcripts: bool,
    pub temperature: f32,
    pub max_tokens: u32,
}
