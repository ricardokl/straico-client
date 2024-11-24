use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CompletionData {
    completions: HashMap<String, Model>,
    overall_price: Price,
    overall_words: Words,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    input: f32,
    output: f32,
    total: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Words {
    input: u32,
    output: u32,
    total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    completion: Completion,
    price: Price,
    words: Words,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    choices: Vec<Choice>,
    object: Box<str>,
    id: Box<str>,
    model: Box<str>,
    created: u64,
    usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    message: Message,
    index: u8,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    content: String,
    role: String,
}

impl CompletionData {
    pub fn get_completion(self) -> Completion {
        let values = self.completions.into_values();
        values.map(|x| x.completion).next().unwrap()
    }
}
