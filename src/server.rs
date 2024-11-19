//! OpenAI API compatibility layer for Straico API
//! 
//! This module provides compatibility with OpenAI's chat completion API format,
//! translating requests to Straico's format and back.

use serde_json::Value;
use std::{borrow::Cow, ops::Deref};

use crate::AppState;
use actix_web::{error::ErrorInternalServerError, post, web, Error, Responder};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use straico::endpoints::completion::completion_request::{CompletionRequest, Prompt};

/// Request format matching OpenAI's chat completion API
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(into = "CompletionRequest")]
pub struct OpenAiRequest<'a> {
    model: Cow<'a, str>,
    messages: Chat<'a>,
    #[serde(alias = "max_completion_tokens")]
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

/// Collection of chat messages in a conversation
#[derive(Deserialize, Clone, Debug)]
pub struct Chat<'a>(pub Vec<Message<'a>>);

impl<'a> Deref for Chat<'a> {
    type Target = Vec<Message<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Chat<'a>> for Prompt<'a> {
    fn from(value: Chat<'a>) -> Self {
        let xml_string = value
            .iter()
            .map(|message| match message.role {
                Role::System => format!("<system>{}</system>", message.content),
                Role::User => format!("<user>{}</user>", message.content),
                Role::Assistant => format!("<assistant>{}</assistant>", message.content),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Prompt::from(Cow::Owned(xml_string))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message<'a> {
    pub role: Role,
    pub content: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl<'a> From<OpenAiRequest<'a>> for CompletionRequest<'a> {
    fn from(value: OpenAiRequest<'a>) -> Self {
        let builder = CompletionRequest::new()
            .models(value.model)
            .message(value.messages);
        match (value.max_tokens, value.temperature) {
            (Some(x), Some(y)) => builder.max_tokens(x).temperature(y).build(),
            (Some(x), None) => builder.max_tokens(x).build(),
            (None, Some(y)) => builder.temperature(y).build(),
            (None, None) => builder.build(),
        }
    }
}

/// Handler for OpenAI-compatible chat completion endpoint
/// 
/// Accepts requests in OpenAI's format and returns responses in a compatible format.
/// When debug mode is enabled, prints both request and response JSON.
/// 
/// # Errors
/// 
/// Returns an error if:
/// - Request JSON cannot be parsed
/// - API call fails
/// - Response cannot be serialized
#[post("/v1/chat/completions")]
pub async fn openai_completion<'a>(
    req: web::Json<Value>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    if data.debug {
        println!("\nReceived request:");
        println!("{}", serde_json::to_string_pretty(&req)?);
    }
    
    let req: OpenAiRequest = serde_json::from_value(req.into_inner())?;
    let client = data.client.clone();
    let response = client
        .completion()
        .bearer_auth(&data.key)
        .json(req)
        .send()
        .map_ok(|c| c.data.get_completion())
        .map_err(|e| ErrorInternalServerError(e))
        .await?;
    
    if data.debug {
        println!("\nReceived response:");
        println!("{}", serde_json::to_string_pretty(&response)?);
    }
    
    Ok(web::Json(response))
}
