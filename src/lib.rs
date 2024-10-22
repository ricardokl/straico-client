#![allow(dead_code)]
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    data: Data,
    success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    completions: HashMap<String, Model>,
    overall_price: Price,
    overall_words: Words,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    input: f32,
    output: f32,
    total: f32,
}

#[derive(Debug, Deserialize)]
pub struct Words {
    input: u32,
    output: u32,
    total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    completion: Completion,
    price: Price,
    words: Words,
}

#[derive(Debug, Deserialize)]
pub struct Completion {
    choices: Vec<Choice>,
    id: String,
    model: String,
    created: usize,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_token: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    message: Message,
    index: u8,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    content: String,
    role: String,
}
