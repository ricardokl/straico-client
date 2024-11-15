use std::ops::Deref;

use crate::AppState;
use actix_web::{error::ErrorInternalServerError, post, web, Error, Responder};
use serde::{Deserialize, Serialize};
use straico::endpoints::completion::{
    completion_request::CompletionRequest, completion_response::Completion,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(alias = "max_tokens")]
    max_completion_tokens: u32,
    temperature: f32,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

impl<'a> OpenAiRequest {
    fn to_completion_request(&'a self) -> CompletionRequest<'a> {
        CompletionRequest::new()
            .models(self.model.as_str())
            .message(self.messages[0].content.deref())
            .max_tokens(self.max_completion_tokens)
            .temperature(self.temperature)
            .build()
    }
}

async fn send(
    req: web::Json<OpenAiRequest>,
    data: web::Data<AppState>,
    model: &str,
) -> reqwest::Result<Completion> {
    let response = data
        .client
        .clone()
        .completion()
        .bearer_auth(&data.key)
        .json(&req.into_inner().to_completion_request())
        .send()
        .await?;
    let parsed = response.json().await?;
    let model_data = parsed.data.get_completion(model);
    Ok(model_data)
}

#[post("/openai/v1/completions")]
pub async fn openai_completion(
    req: web::Json<OpenAiRequest>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let model = req.model.clone();
    Ok(web::Json(
        send(req, data, &model)
            .await
            .map_err(|e| ErrorInternalServerError(e))?,
    ))
}
