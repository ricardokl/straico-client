#![allow(dead_code)]
use crate::client::{ApiKeySet, IntoStraicoClient, PayloadSet, StraicoRequestBuilder};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct CompletionsData {
    completions: HashMap<String, Model>,
    overall_price: Price,
    overall_words: Words,
}

#[derive(Deserialize)]
pub struct Price {
    input: f32,
    output: f32,
    total: f32,
}

#[derive(Deserialize)]
pub struct Words {
    input: u32,
    output: u32,
    total: u32,
}

#[derive(Deserialize)]
pub struct Model {
    completion: Completion,
    price: Price,
    words: Words,
}

#[derive(Deserialize)]
pub struct Completion {
    choices: Vec<Choice>,
    id: String,
    model: String,
    created: usize,
    usage: Usage,
}

#[derive(Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_token: u32,
}

#[derive(Deserialize)]
pub struct Choice {
    message: Message,
    index: u8,
    finish_reason: String,
}

#[derive(Deserialize)]
pub struct Message {
    content: String,
    role: String,
}

#[derive(Serialize)]
pub struct CompletionRequest {
    models: Vec<String>,
    message: String,
    file_urls: Option<Vec<String>>,
    youtube_urls: Option<Vec<String>>,
    display_transcripts: Option<bool>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

struct ModelsSet(Vec<String>);
struct MessageSet(String);
struct ModelsNotSet;
struct MessageNotSet;

struct CompletionRequestBuilder<T, K> {
    models: T,
    message: K,
    file_urls: Option<Vec<String>>,
    youtube_urls: Option<Vec<String>>,
    display_transcripts: Option<bool>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

impl CompletionRequest {
    fn new() -> CompletionRequestBuilder<ModelsNotSet, MessageNotSet> {
        CompletionRequestBuilder {
            models: ModelsNotSet,
            message: MessageNotSet,
            file_urls: None,
            youtube_urls: None,
            display_transcripts: None,
            temperature: None,
            max_tokens: None,
        }
    }
}

impl<T> CompletionRequestBuilder<ModelsNotSet, T> {
    fn models(self, models: &[&str]) -> CompletionRequestBuilder<ModelsSet, T> {
        CompletionRequestBuilder {
            models: ModelsSet(models.iter().map(|s| s.to_string()).collect()),
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            message: self.message,
        }
    }

    fn model(self, model: &str) -> CompletionRequestBuilder<ModelsSet, T> {
        CompletionRequestBuilder {
            models: ModelsSet(vec![model.to_string()]),
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            message: self.message,
        }
    }
}

impl<T> CompletionRequestBuilder<T, MessageNotSet> {
    fn message(self, message: &str) -> CompletionRequestBuilder<T, MessageSet> {
        CompletionRequestBuilder {
            models: self.models,
            message: MessageSet(message.into()),
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}

impl<T, K> CompletionRequestBuilder<T, K> {
    fn file_urls(self, file_urls: Vec<&str>) -> CompletionRequestBuilder<T, K> {
        CompletionRequestBuilder {
            file_urls: Some(file_urls.iter().map(|s| s.to_string()).collect()),
            ..self
        }
    }

    fn youtube_urls(self, youtube_urls: Vec<&str>) -> CompletionRequestBuilder<T, K> {
        CompletionRequestBuilder {
            youtube_urls: Some(youtube_urls.iter().map(|s| s.to_string()).collect()),
            ..self
        }
    }

    fn display_transcripts(self, display_transcripts: bool) -> CompletionRequestBuilder<T, K> {
        CompletionRequestBuilder {
            display_transcripts: Some(display_transcripts),
            ..self
        }
    }

    fn temperature(self, temperature: f32) -> CompletionRequestBuilder<T, K> {
        CompletionRequestBuilder {
            temperature: Some(temperature),
            ..self
        }
    }

    fn max_tokens(self, max_tokens: u32) -> CompletionRequestBuilder<T, K> {
        CompletionRequestBuilder {
            max_tokens: Some(max_tokens),
            ..self
        }
    }
}

impl CompletionRequestBuilder<ModelsSet, MessageSet> {
    fn build(self) -> CompletionRequest {
        CompletionRequest {
            models: self.models.0,
            message: self.message.0,
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }

    fn bearer_auth<T: Display>(
        self,
        key: T,
    ) -> StraicoRequestBuilder<ApiKeySet, PayloadSet<CompletionRequest>> {
        let payload = self.build();
        ReqwestClient::new()
            .to_straico()
            .completion()
            .bearer_auth(key)
            .json(&payload)
    }
}
