use futures::FutureExt;
use reqwest::{multipart::Form, Client, RequestBuilder, Response};
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

pub trait IntoStraicoClient {
    fn to_straico(self) -> StraicoClient;
}

impl IntoStraicoClient for Client {
    fn to_straico(self) -> StraicoClient {
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
        Client::new().to_straico()
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

type FormResult<T> = Result<StraicoRequestBuilder<T, PayloadSet, FileData>, std::io::Error>;
impl<T> StraicoRequestBuilder<T, FileRequest, FileData> {
    pub async fn multipart<U: AsRef<Path>>(self, file: U) -> FormResult<T> {
        let form = Form::new().file("file", file).await?;
        Ok(self.with(|b| b.multipart(form)))
    }
}

impl<K, T: Serialize, V> StraicoRequestBuilder<K, T, V> {
    pub fn json(self, payload: &T) -> StraicoRequestBuilder<K, PayloadSet, V> {
        self.with(|b| b.json(payload))
    }
}

pub struct StraicoResponse<T>(Response, PhantomData<T>);
impl<V: for<'a> Deserialize<'a>> StraicoRequestBuilder<ApiKeySet, PayloadSet, V> {
    pub fn send(self) -> impl Future<Output = reqwest::Result<StraicoResponse<V>>> {
        self.0.send().map(|x| x.map(|y| StraicoResponse::from(y)))
    }
}

impl<V> From<Response> for StraicoResponse<V> {
    fn from(response: Response) -> StraicoResponse<V> {
        StraicoResponse(response, PhantomData)
    }
}

// Can fail if there is an error, should implement it
// Maybe make the response an enum with a success and error variant
impl<V: for<'a> Deserialize<'a>> StraicoResponse<V> {
    pub fn json(self) -> impl Future<Output = reqwest::Result<ApiResponseData<V>>> {
        self.0.json()
    }
}

impl<S, T, V> StraicoRequestBuilder<S, T, V> {
    fn with<F, R, U>(self, f: F) -> StraicoRequestBuilder<U, R, V>
    where
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        StraicoRequestBuilder(f(self.0), PhantomData, PhantomData, PhantomData)
    }
}

impl StraicoClient {
    fn with<F, S, U, V>(self, f: F) -> StraicoRequestBuilder<S, U, V>
    where
        F: FnOnce(Client) -> RequestBuilder,
    {
        StraicoRequestBuilder(f(self.0), PhantomData, PhantomData, PhantomData)
    }
}
