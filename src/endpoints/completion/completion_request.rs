use crate::client::{ApiKeySet, IntoStraicoClient, PayloadSet, StraicoRequestBuilder};
use crate::endpoints::completion::completion_response::CompletionData;
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize)]
pub struct CompletionRequest<'a> {
    models: RequestModels<'a>,
    message: &'a str,
    file_urls: Option<Vec<&'a str>>,
    youtube_urls: Option<Vec<&'a str>>,
    display_transcripts: Option<bool>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
pub struct RequestModels<'a>(
    #[serde(skip_serializing_if = "Option::is_none")] Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<&'a str>,
);

impl Default for RequestModels<'_> {
    fn default() -> Self {
        Self(Some("openai/gpt-3.5-turbo-0125"), None, None, None)
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for RequestModels<'a>
where
    [(); N]: Max4,
{
    fn from(arr: [&'a str; N]) -> Self {
        let [a, b, c, d] = std::array::from_fn(|i| arr.get(i).map(|x| *x));
        Self(a, b, c, d)
    }
}

trait Max4 {}
impl Max4 for [(); 1] {}
impl Max4 for [(); 2] {}
impl Max4 for [(); 3] {}
impl Max4 for [(); 4] {}

impl<'a> From<&'a str> for RequestModels<'a> {
    fn from(value: &'a str) -> Self {
        [value; 1].into()
    }
}

pub struct ModelsSet<'a>(RequestModels<'a>);
pub struct MessageSet<'a>(&'a str);
pub struct ModelsNotSet;
pub struct MessageNotSet;

#[derive(Debug)]
pub struct TooManyModels;

impl<'a> TryFrom<&[&'a str]> for RequestModels<'a> {
    type Error = TooManyModels;

    fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
        match value.len() {
            0 => Ok(RequestModels::default()),
            1..=4 => {
                let [a, b, c, d] = std::array::from_fn(|i| value.get(i).map(|x| *x));
                Ok(RequestModels(a, b, c, d))
            }
            _ => Err(TooManyModels),
        }
    }
}

impl<'a> TryFrom<Vec<&'a str>> for RequestModels<'a> {
    type Error = TooManyModels;

    fn try_from(value: Vec<&'a str>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<'a> From<RequestModels<'a>> for ModelsSet<'a> {
    fn from(value: RequestModels<'a>) -> Self {
        Self(value)
    }
}

pub struct CompletionRequestBuilder<'a, T, K> {
    models: T,
    message: K,
    file_urls: Option<Vec<&'a str>>,
    youtube_urls: Option<Vec<&'a str>>,
    display_transcripts: Option<bool>,
    temperature: f32,
    max_tokens: u32,
}

impl<'a> CompletionRequest<'a> {
    pub fn new() -> CompletionRequestBuilder<'a, ModelsNotSet, MessageNotSet> {
        CompletionRequestBuilder {
            models: ModelsNotSet,
            message: MessageNotSet,
            file_urls: None,
            youtube_urls: None,
            display_transcripts: None,
            temperature: 0.0,
            max_tokens: 32000,
        }
    }
}

impl<'a, T> CompletionRequestBuilder<'a, ModelsNotSet, T> {
    pub fn models<M>(self, models: M) -> CompletionRequestBuilder<'a, ModelsSet<'a>, T>
    where
        M: Into<RequestModels<'a>>,
    {
        CompletionRequestBuilder {
            models: ModelsSet(models.into()),
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            message: self.message,
        }
    }
}

impl<'a, T> CompletionRequestBuilder<'a, T, MessageNotSet> {
    pub fn message<M>(self, message: M) -> CompletionRequestBuilder<'a, T, MessageSet<'a>>
    where
        M: Into<&'a str>,
    {
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

impl<'a, T, K> CompletionRequestBuilder<'a, T, K> {
    pub fn file_urls(self, file_urls: &[&'a str]) -> CompletionRequestBuilder<'a, T, K> {
        CompletionRequestBuilder {
            file_urls: Some(file_urls.into()),
            ..self
        }
    }

    pub fn youtube_urls(self, youtube_urls: &[&'a str]) -> CompletionRequestBuilder<'a, T, K> {
        CompletionRequestBuilder {
            youtube_urls: Some(youtube_urls.into()),
            ..self
        }
    }

    pub fn display_transcripts(
        self,
        display_transcripts: bool,
    ) -> CompletionRequestBuilder<'a, T, K> {
        CompletionRequestBuilder {
            display_transcripts: Some(display_transcripts),
            ..self
        }
    }

    pub fn temperature(self, temperature: f32) -> CompletionRequestBuilder<'a, T, K> {
        CompletionRequestBuilder {
            temperature,
            ..self
        }
    }

    pub fn max_tokens(self, max_tokens: u32) -> CompletionRequestBuilder<'a, T, K> {
        CompletionRequestBuilder { max_tokens, ..self }
    }
}

impl<'a> CompletionRequestBuilder<'a, ModelsSet<'a>, MessageSet<'a>> {
    pub fn build(self) -> CompletionRequest<'a> {
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

    pub fn bearer_auth<T: Display>(
        self,
        key: T,
    ) -> StraicoRequestBuilder<ApiKeySet, PayloadSet, CompletionData> {
        let payload = self.build();
        ReqwestClient::new()
            .to_straico()
            .completion()
            .bearer_auth(key)
            .json(&payload)
    }
}
