use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CompletionData {
    pub completions: HashMap<String, Model>,
    overall_price: Price,
    overall_words: Words,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Price {
    input: f32,
    output: f32,
    total: f32,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Words {
    input: u32,
    output: u32,
    total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub completion: Completion,
    price: Price,
    words: Words,
}

#[derive(Serialize, Deserialize)]
pub struct Completion {
    pub choices: Vec<Choice>,
    pub object: Box<str>,
    pub id: Box<str>,
    pub model: Box<str>,
    pub created: u64,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub index: u8,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

impl CompletionData {
    pub fn get_completion(self, model: &str) -> Completion {
        self.completions
            .into_iter()
            .find(|(m, _)| m == model)
            .map(|(_, c)| c.completion)
            .unwrap()
    }
}
