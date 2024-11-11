use reqwest::multipart::Form;
use reqwest::Client as ReqwestClient;
use reqwest::RequestBuilder as ReqwestBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Error;
use std::marker::PhantomData;
use std::path::Path;

use crate::common::ApiResponse;
use crate::endpoints::{
    completion::completion_request::CompletionRequest,
    completion::completion_response::CompletionData,
    file::{FileData, FileRequest},
    image::{ImageData, ImageRequest},
    model::ModelData,
    user::UserData,
};
use crate::{GetEndpoint, PostEndpoint};

#[allow(dead_code)]
pub struct NoApiKey;
#[allow(dead_code)]
pub struct ApiKeySet;
#[allow(dead_code)]
pub struct PayloadSet;

pub struct StraicoRequestBuilder<Api, Payload, Response> {
    builder: ReqwestBuilder,
    _response: PhantomData<Response>,
    _payload: PhantomData<Payload>,
    _api: PhantomData<Api>,
}

pub trait IntoStraicoClient {
    fn to_straico(self) -> StraicoClient;
}

impl IntoStraicoClient for ReqwestClient {
    fn to_straico(self) -> StraicoClient {
        StraicoClient { client: self }
    }
}

pub struct StraicoClient {
    client: ReqwestClient,
}

impl<'a> StraicoClient {
    pub fn new() -> StraicoClient {
        ReqwestClient::new().to_straico()
    }

    pub fn completion(
        self,
    ) -> StraicoRequestBuilder<NoApiKey, CompletionRequest<'a>, CompletionData> {
        self.with(|c| c.post(PostEndpoint::Completion.as_ref()))
    }

    pub fn image(self) -> StraicoRequestBuilder<NoApiKey, ImageRequest, ImageData> {
        self.with(|c| c.post(PostEndpoint::Image.as_ref()))
    }

    pub fn file(self) -> StraicoRequestBuilder<NoApiKey, FileRequest, FileData> {
        self.with(|c| c.post(PostEndpoint::File.as_ref()))
    }

    pub fn models(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, ModelData> {
        self.with(|c| c.get(GetEndpoint::Models.as_ref()))
    }

    pub fn user(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, UserData> {
        self.with(|c| c.get(GetEndpoint::User.as_ref()))
    }
}

impl<T, U> StraicoRequestBuilder<NoApiKey, T, U> {
    pub fn bearer_auth<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T, U> {
        self.with(|b| b.bearer_auth(api_key))
    }
}

impl<T> StraicoRequestBuilder<T, FileRequest, FileData> {
    pub async fn form<U: AsRef<Path>>(
        self,
        file: U,
    ) -> Result<StraicoRequestBuilder<T, PayloadSet, FileData>, Error> {
        let form = Form::new().file("file", file).await?;
        Ok(self.with(|b| b.multipart(form)))
    }
}

impl<K, T: Serialize, V> StraicoRequestBuilder<K, T, V> {
    pub fn json(self, payload: &T) -> StraicoRequestBuilder<K, PayloadSet, V> {
        self.with(|b| b.json(payload))
    }
}

impl<V: for<'a> Deserialize<'a>> StraicoRequestBuilder<ApiKeySet, PayloadSet, V> {
    pub async fn send(self) -> ApiResponse<V> {
        let response = self.builder.send().await?;
        Ok(response.json().await?)
    }
}

impl<S, T, V> StraicoRequestBuilder<S, T, V> {
    fn with<F, R, U>(self, f: F) -> StraicoRequestBuilder<U, R, V>
    where
        F: FnOnce(ReqwestBuilder) -> ReqwestBuilder,
    {
        StraicoRequestBuilder {
            builder: f(self.builder),
            _response: PhantomData,
            _payload: PhantomData,
            _api: PhantomData,
        }
    }
}

impl StraicoClient {
    fn with<F, S, U, V>(self, f: F) -> StraicoRequestBuilder<S, U, V>
    where
        F: FnOnce(ReqwestClient) -> ReqwestBuilder,
    {
        StraicoRequestBuilder {
            builder: f(self.client),
            _response: PhantomData,
            _payload: PhantomData,
            _api: PhantomData,
        }
    }
}
