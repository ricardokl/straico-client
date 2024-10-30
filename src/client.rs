#![allow(dead_code)]
use reqwest::Client as ReqwestClient;
use reqwest::RequestBuilder as ReqwestBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::marker::PhantomData;

use crate::common::ApiResponse;
use crate::endpoints::{
    completion::{CompletionRequest, CompletionsData},
    file::FileData,
    image::{ImageData, ImageRequest},
    model::ModelData,
    user::UserData,
};
use crate::{GetEndpoint, PostEndpoint};

pub struct NoApiKey;
pub struct ApiKeySet;
pub struct PayloadSet<T = ()>(PhantomData<T>);
pub struct NoPayload<T>(PhantomData<T>);
#[derive(Serialize)]
pub struct FileRequest;

pub struct StraicoRequestBuilder<Api = (), Payload = ()> {
    builder: ReqwestBuilder,
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

impl StraicoClient {
    pub fn completion(self) -> StraicoRequestBuilder<NoApiKey, CompletionRequest> {
        self.with(|c| c.post(PostEndpoint::Completion.as_ref()))
    }

    pub fn image(self) -> StraicoRequestBuilder<NoApiKey, ImageRequest> {
        self.with(|c| c.post(PostEndpoint::Image.as_ref()))
    }

    pub fn file(self) -> StraicoRequestBuilder<NoApiKey, FileRequest> {
        self.with(|c| c.post(PostEndpoint::File.as_ref()))
    }

    pub fn models(self) -> StraicoRequestBuilder<NoApiKey, NoPayload<ModelData>> {
        self.with(|c| c.get(GetEndpoint::Models.as_ref()))
    }

    pub fn user(self) -> StraicoRequestBuilder<NoApiKey, NoPayload<UserData>> {
        self.with(|c| c.get(GetEndpoint::User.as_ref()))
    }
}

impl<T> StraicoRequestBuilder<NoApiKey, T> {
    pub fn bearer_auth<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T> {
        self.with(|b| b.bearer_auth(api_key))
    }
}

impl<K, T: Serialize> StraicoRequestBuilder<K, T> {
    pub fn json(self, payload: &T) -> StraicoRequestBuilder<K, PayloadSet<T>> {
        self.with(|b| b.json(payload))
    }
}

impl<T: for<'a> Deserialize<'a>> StraicoRequestBuilder<ApiKeySet, NoPayload<T>> {
    pub async fn send(self) -> ApiResponse<T> {
        let response = self.builder.send().await?;
        Ok(response.json().await?)
    }
}

macro_rules! define_send_methods {
    ($(($payload:ty, $response:ty)),* $(,)?) => {
        $(
            impl StraicoRequestBuilder<ApiKeySet, PayloadSet<$payload>> {
                pub async fn send(self) -> ApiResponse<$response> {
                    let response = self.builder.send().await?;
                    Ok(response.json().await?)
                }
            }
        )*
    };
}

define_send_methods! {
    (CompletionRequest, CompletionsData),
    (ImageRequest, ImageData),
    (FileRequest, FileData),
}

impl<State, T> StraicoRequestBuilder<State, T> {
    fn with<F, S, U>(self, f: F) -> StraicoRequestBuilder<S, U>
    where
        F: FnOnce(ReqwestBuilder) -> ReqwestBuilder,
    {
        StraicoRequestBuilder {
            builder: f(self.builder),
            _payload: PhantomData,
            _api: PhantomData,
        }
    }
}

impl StraicoClient {
    fn with<F, S, U>(self, f: F) -> StraicoRequestBuilder<S, U>
    where
        F: FnOnce(ReqwestClient) -> ReqwestBuilder,
    {
        StraicoRequestBuilder {
            builder: f(self.client),
            _payload: PhantomData,
            _api: PhantomData,
        }
    }
}
