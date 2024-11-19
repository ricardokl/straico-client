use futures::TryFutureExt;
use reqwest::{multipart::Form, Client, RequestBuilder, Response, IntoResponse};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, future::Future, marker::PhantomData, path::Path};

use crate::endpoints::{
    completion::completion_request::CompletionRequest,
    completion::completion_response::CompletionData,
    file::{FileData, FileRequest},
    image::{ImageData, ImageRequest},
    model::ModelData,
    user::UserData,
    ApiResponseData,
};
use crate::{GetEndpoint, PostEndpoint};

pub struct NoApiKey;
pub struct ApiKeySet;
pub struct PayloadSet;

pub struct StraicoRequestBuilder<Api, Payload, Response>(
    RequestBuilder,
    PhantomData<Response>,
    PhantomData<Payload>,
    PhantomData<Api>,
);

impl Into<StraicoClient> for Client {
    fn into(self) -> StraicoClient {
        StraicoClient(self)
    }
}

pub struct StraicoClient(Client);

impl Clone for StraicoClient {
    fn clone(&self) -> Self {
        StraicoClient(self.0.clone())
    }
}

impl<'a> StraicoClient {
    pub fn new() -> StraicoClient {
        Client::new().into()
    }

    pub fn completion(
        self,
    ) -> StraicoRequestBuilder<NoApiKey, CompletionRequest<'a>, CompletionData> {
        self.0.post(PostEndpoint::Completion.as_ref()).into()
    }

    pub fn image(self) -> StraicoRequestBuilder<NoApiKey, ImageRequest, ImageData> {
        self.0.post(PostEndpoint::Image.as_ref()).into()
    }

    pub fn file(self) -> StraicoRequestBuilder<NoApiKey, FileRequest, FileData> {
        self.0.post(PostEndpoint::File.as_ref()).into()
    }

    pub fn models(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, ModelData> {
        self.0.get(GetEndpoint::Models.as_ref()).into()
    }

    pub fn user(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, UserData> {
        self.0.get(GetEndpoint::User.as_ref()).into()
    }
}

impl<T, U> StraicoRequestBuilder<NoApiKey, T, U> {
    pub fn bearer_auth<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T, U> {
        self.0.bearer_auth(api_key).into()
    }
}

type FormResult<T> = Result<StraicoRequestBuilder<T, PayloadSet, FileData>, std::io::Error>;
impl<T> StraicoRequestBuilder<T, FileRequest, FileData> {
    pub async fn multipart<U: AsRef<Path>>(self, file: U) -> FormResult<T> {
        let form = Form::new().file("file", file).await?;
        Ok(self.0.multipart(form).into())
    }
}

impl<K, T: Serialize, V> StraicoRequestBuilder<K, T, V> {
    pub fn json<U: Into<T>>(self, payload: U) -> StraicoRequestBuilder<K, PayloadSet, V> {
        self.0.json(&payload.into()).into()
    }
}

impl<V: for<'a> Deserialize<'a>> StraicoRequestBuilder<ApiKeySet, PayloadSet, V> {
    pub fn send(self) -> impl Future<Output = reqwest::Result<ApiResponseData<V>>> {
        self.0.send().and_then(Response::json)
    }

    pub fn send_stream(self) -> impl Future<Output = reqwest::Result<Response>> {
        self.0.send()
    }
}

impl<T, U, V> From<RequestBuilder> for StraicoRequestBuilder<T, U, V> {
    fn from(value: RequestBuilder) -> Self {
        StraicoRequestBuilder(value, PhantomData, PhantomData, PhantomData)
    }
}
