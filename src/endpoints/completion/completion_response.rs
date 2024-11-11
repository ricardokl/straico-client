use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct CompletionData {
    completions: HashMap<String, Model>,
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Model {
    completion: Completion,
    price: Price,
    words: Words,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Completion {
    choices: Vec<Choice>,
    id: String,
    model: String,
    created: usize,
    usage: Usage,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_token: u32,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Choice {
    message: Message,
    index: u8,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,
    role: String,
}
