use reqwest::Client as ReqwestClient;
use reqwest::RequestBuilder as ReqwestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::marker::PhantomData;

use crate::common::ApiResponse;
use crate::endpoints::{completion::CompletionRequest, image::ImageRequest};
use crate::{GetEndpoint, PostEndpoint};

pub struct ApiKeyNotSet;
pub struct ApiKeySet;
pub struct PayloadSet;
pub struct UserNoPayload;
pub struct ModelsNoPayload;

pub struct StraicoRequestBuilder<State, T = ()> {
    client: ReqwestBuilder,
    _payload: PhantomData<T>,
    _state: PhantomData<State>,
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

// impl StraicoClient {
//     pub fn file(self) -> StraicoRequestBuilder<ApiKeyNotSet, ImageRequest> {
//         StraicoRequestBuilder::<ApiKeyNotSet, ImageRequest> {
//             client: self.client.post(PostEndpoint::File.as_ref()),
//             _payload: PhantomData,
//             _state: PhantomData,
//         }
//     }

//     pub fn create_image(&self) -> StraicoRequestBuilder<ApiKeyNotSet, ImageRequest> {
//         StraicoRequestBuilder::<ApiKeyNotSet, ImageRequest> {
//             client: self.client.post(PostEndpoint::Image.as_ref()),
//             _payload: PhantomData,
//             _state: PhantomData,
//         }
//     }

//     pub fn completion(&self) -> StraicoRequestBuilder<ApiKeyNotSet, CompletionRequest> {
//         StraicoRequestBuilder::<ApiKeyNotSet, CompletionRequest> {
//             client: self.client.post(PostEndpoint::Completion.as_ref()),
//             _payload: PhantomData,
//             _state: PhantomData,
//         }
//     }

//     pub fn user(&self) -> StraicoRequestBuilder<ApiKeyNotSet> {
//         StraicoRequestBuilder::<ApiKeyNotSet> {
//             client: self.client.get(GetEndpoint::User.as_ref()),
//             _payload: PhantomData,
//             _state: PhantomData,
//         }
//     }

//     pub fn models(&self) -> StraicoRequestBuilder<ApiKeyNotSet> {
//         StraicoRequestBuilder::<ApiKeyNotSet> {
//             client: self.client.get(GetEndpoint::Models.as_ref()),
//             _payload: PhantomData,
//             _state: PhantomData,
//         }
//     }
// }

impl<T> StraicoRequestBuilder<ApiKeyNotSet, T> {
    pub fn set_key<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T> {
        StraicoRequestBuilder::<ApiKeySet, T> {
            client: self.client.bearer_auth(api_key),
            _payload: PhantomData,
            _state: PhantomData,
        }
    }
}

impl StraicoRequestBuilder<ApiKeySet> {
    pub async fn send<R: DeserializeOwned>(self) -> ApiResponse<R> {
        let response = self.client.send().await?;

        Ok(response.json().await?)
    }
}

impl<T: Serialize> StraicoRequestBuilder<ApiKeySet, T> {
    pub fn with_payload(self, payload: &T) -> StraicoRequestBuilder<PayloadSet, T> {
        StraicoRequestBuilder::<PayloadSet, T> {
            client: self.client.json(payload),
            _payload: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<T: Serialize> StraicoRequestBuilder<PayloadSet, T> {
    pub async fn send<R: DeserializeOwned>(self) -> ApiResponse<R> {
        let response = self.client.send().await?;

        Ok(response.json().await?)
    }
}

macro_rules! define_endpoints {
    ($($ident:ident ($name:ident, $endpoint:path, $payload:ty)),* $(,)?) => {
        impl StraicoClient {
            $(
                pub fn $name(&self) -> StraicoRequestBuilder<ApiKeyNotSet, $payload> {
                    StraicoRequestBuilder::<ApiKeyNotSet, $payload> {
                        client: self.client.$ident($endpoint.as_ref()),
                        _payload: PhantomData,
                        _state: PhantomData,
                    }
                }
            )*
        }
    };
}

define_endpoints! {
    post(file, PostEndpoint::File, ImageRequest),
    post(create_image, PostEndpoint::Image, ImageRequest),
    post(completion, PostEndpoint::Completion, CompletionRequest),
    get(user, GetEndpoint::User, UserNoPayload),
    get(models, GetEndpoint::Models, ModelsNoPayload),
}
