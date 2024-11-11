#![allow(dead_code)]
use crate::client::StraicoClient;
use crate::endpoints::completion::completion_request::CompletionRequest;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenAiRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    max_completion_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
pub struct Message<'a> {
    role: Role,
    content: &'a str,
}

#[derive(Deserialize)]
pub enum Role {
    User,
    Assistant,
}

async fn openai_completion<'a>(req: web::Json<OpenAiRequest<'a>>) -> HttpResponse {
    let api_key = "test";
    let request = CompletionRequest::new()
        .models(req.model)
        .message(req.messages[0].content)
        .max_tokens(req.max_completion_tokens)
        .temperature(req.temperature)
        .build();
    match StraicoClient::new()
        .completion()
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::from_error(error),
    }
}
