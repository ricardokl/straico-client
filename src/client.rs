use reqwest::Client as ReqwestClient;
use reqwest::RequestBuilder as ReqwestBuilder;
use serde::Serialize;
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

pub struct ApiKeyNotSet;
pub struct ApiKeySet;
pub struct PayloadSet;
pub struct UserNoPayload;
pub struct ModelsNoPayload;
pub struct FileRequest;

pub struct StraicoRequestBuilder<State, T = ()> {
    builder: ReqwestBuilder,
    _payload: PhantomData<T>,
    _state: PhantomData<State>,
}

impl<S, T> Default for StraicoRequestBuilder<S, T> {
    fn default() -> Self {
        Self {
            builder: ReqwestClient::new().post(PostEndpoint::Completion.as_ref()),
            _payload: PhantomData,
            _state: PhantomData,
        }
    }
}

pub trait IntoStraicoClient {
    fn to_straico(self) -> StraicoClient;
}

impl IntoStraicoClient for ReqwestClient {
    fn to_straico(self) -> StraicoClient {
        StraicoClient { client: self }
    }
}

impl From<ReqwestClient> for StraicoClient {
    fn from(client: ReqwestClient) -> Self {
        client.to_straico()
    }
}

pub struct StraicoClient {
    client: ReqwestClient,
}

impl<T> StraicoRequestBuilder<ApiKeyNotSet, T> {
    pub fn set_key<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T> {
        self.with(|b| b.bearer_auth(api_key))
    }
}

impl<T: Serialize> StraicoRequestBuilder<ApiKeySet, T> {
    pub fn set_payload(self, payload: &T) -> StraicoRequestBuilder<PayloadSet, T> {
        self.with(|b| b.json(payload))
    }
}

macro_rules! define_endpoints {
    ($($ident:ident ($name:ident, $endpoint:path, $payload:ty)),* $(,)?) => {
        impl StraicoClient {
            $(
                pub fn $name(self) -> StraicoRequestBuilder<ApiKeyNotSet, $payload> {
                    StraicoRequestBuilder::<ApiKeyNotSet, $payload> {
                        builder: self.client.$ident($endpoint.as_ref()),
                        ..Default::default()
                    }
                }
            )*
        }
    };
}

define_endpoints! {
    post(completion, PostEndpoint::Completion, CompletionRequest),
    post(create_image, PostEndpoint::Image, ImageRequest),
    post(file, PostEndpoint::File, ImageRequest),
    get(models, GetEndpoint::Models, ModelsNoPayload),
    get(user, GetEndpoint::User, UserNoPayload),
}

macro_rules! define_send_methods {
    ($(($state:ty, $payload:ty, $response:ty)),* $(,)?) => {
        $(
            impl StraicoRequestBuilder<$state, $payload> {
                pub async fn send(self) -> ApiResponse<$response> {
                    let response = self.builder.send().await?;
                    Ok(response.json().await?)
                }
            }
        )*
    };
}

define_send_methods! {
    (ApiKeySet, UserNoPayload, UserData),
    (ApiKeySet, ModelsNoPayload, ModelData),
    (PayloadSet, CompletionRequest, CompletionsData),
    (PayloadSet, ImageRequest, ImageData),
    (PayloadSet, FileRequest, FileData),
}

impl<State, T> StraicoRequestBuilder<State, T> {
    pub fn with<F, S, U>(self, f: F) -> StraicoRequestBuilder<S, U>
    where
        F: FnOnce(ReqwestBuilder) -> ReqwestBuilder,
    {
        StraicoRequestBuilder {
            builder: f(self.builder),
            ..Default::default()
        }
    }
}
