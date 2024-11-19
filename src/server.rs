use std::{borrow::Cow, ops::Deref};

use crate::AppState;
use actix_web::{error::ErrorInternalServerError, post, web, Error, Responder};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use straico::endpoints::completion::completion_request::{CompletionRequest, Prompt};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(into = "CompletionRequest")]
struct OpenAiRequest<'a> {
    model: Cow<'a, str>,
    messages: Chat<'a>,
    #[serde(alias = "max_completion_tokens")]
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Deserialize, Clone, Debug)]
struct Chat<'a>(Vec<Message<'a>>);

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
                Role::User => format!("<user>{}</user>", message.content),
                Role::Assistant => format!("<assistant>{}</assistant>", message.content),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Prompt::from(Cow::Owned(xml_string))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message<'a> {
    role: Role,
    content: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
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

#[post("/openai/v1/chat/completions")]
pub async fn openai_completion<'a>(
    req: web::Json<OpenAiRequest<'a>>,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let client = data.client.clone();
    let response = client
        .completion()
        .bearer_auth(&data.key)
        .json(req.into_inner())
        .send()
        .map_ok(|c| c.data.get_completion())
        .map_err(|e| ErrorInternalServerError(e))
        .await?;
    Ok(web::Json(response))
}
