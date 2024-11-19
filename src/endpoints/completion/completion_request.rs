use crate::client::{ApiKeySet, PayloadSet, StraicoClient, StraicoRequestBuilder};
use crate::endpoints::completion::completion_response::CompletionData;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Display;

#[derive(Serialize)]
pub struct CompletionRequest<'a> {
    models: RequestModels<'a>,
    message: Prompt<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_urls: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    youtube_urls: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_transcripts: Option<bool>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Serialize, Clone)]
pub struct Prompt<'a>(Cow<'a, str>);

impl<'a> From<Cow<'a, str>> for Prompt<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Prompt(value)
    }
}

#[derive(Serialize)]
pub struct RequestModels<'a>(
    #[serde(skip_serializing_if = "Option::is_none")] Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<Cow<'a, str>>,
);

impl Default for RequestModels<'_> {
    fn default() -> Self {
        Self(
            Some(Cow::Borrowed("openai/gpt-3.5-turbo-0125")),
            None,
            None,
            None,
        )
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for RequestModels<'a>
where
    [(); N]: Max4,
{
    fn from(arr: [&'a str; N]) -> Self {
        let [a, b, c, d] = std::array::from_fn(|i| arr.get(i).map(|x| Cow::Borrowed(*x)));
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

impl<'a> From<Cow<'a, str>> for RequestModels<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        RequestModels(Some(value), None, None, None)
    }
}

// pub struct ModelsSet<'a>(RequestModels<'a>);
// #[derive(Serialize)]
// #[serde(transparent)]
// pub struct MessageSet<'a>(&'a str);
pub struct ModelsNotSet;
pub struct MessageNotSet;

// impl<'a> From<RequestModels<'a>> for ModelsSet<'a> {
//     fn from(value: RequestModels<'a>) -> Self {
//         Self(value)
//     }
// }

type MessageSet<'a> = Prompt<'a>;
type ModelsSet<'a> = RequestModels<'a>;

pub struct CompletionRequestBuilder<'a, T, K> {
    models: T,
    message: K,
    file_urls: Option<Vec<&'a str>>,
    youtube_urls: Option<Vec<&'a str>>,
    display_transcripts: Option<bool>,
    temperature: f32,
    max_tokens: Option<u32>,
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
            max_tokens: None,
        }
    }
}

impl<'a, T> CompletionRequestBuilder<'a, ModelsNotSet, T> {
    pub fn models<M>(self, models: M) -> CompletionRequestBuilder<'a, ModelsSet<'a>, T>
    where
        M: Into<RequestModels<'a>>,
    {
        CompletionRequestBuilder {
            models: models.into(),
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
        M: Into<Prompt<'a>>,
    {
        CompletionRequestBuilder {
            models: self.models,
            message: message.into(),
            file_urls: self.file_urls,
            youtube_urls: self.youtube_urls,
            display_transcripts: self.display_transcripts,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}

impl<'a, T, K> CompletionRequestBuilder<'a, T, K> {
    pub fn file_urls(mut self, file_urls: &[&'a str]) -> CompletionRequestBuilder<'a, T, K> {
        self.file_urls = Some(file_urls.into());
        self
    }

    pub fn youtube_urls(mut self, youtube_urls: &[&'a str]) -> CompletionRequestBuilder<'a, T, K> {
        self.youtube_urls = Some(youtube_urls.into());
        self
    }

    pub fn display_transcripts(
        mut self,
        display_transcripts: bool,
    ) -> CompletionRequestBuilder<'a, T, K> {
        self.display_transcripts = Some(display_transcripts);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> CompletionRequestBuilder<'a, T, K> {
        self.temperature = temperature;
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> CompletionRequestBuilder<'a, T, K> {
        self.max_tokens = Some(max_tokens);
        self
    }
}

impl<'a> CompletionRequestBuilder<'a, ModelsSet<'a>, MessageSet<'a>> {
    pub fn build(self) -> CompletionRequest<'a> {
        CompletionRequest {
            models: self.models,
            message: self.message,
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
        let client: StraicoClient = StraicoClient::new();
        client.completion().bearer_auth(key).json(payload)
    }
}
