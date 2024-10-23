use core::fmt;
use reqwest::Url;

use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::ApiResponse;
use crate::endpoints::completion::{CompletionRequest, CompletionsData};
use crate::endpoints::image::{ImageData, ImageRequest};
use crate::{Endpoint, BASE_URL};

pub struct Client<A> {
    client: ReqwestClient,
    api_key: A,
}

impl<A: fmt::Display> Client<A> {
    pub fn new(api_key: A) -> Self
    where
        A: fmt::Display,
    {
        Self {
            client: ReqwestClient::new(),
            api_key,
        }
    }

    pub async fn post<T, R>(&self, endpoint: &Endpoint, payload: &T) -> ApiResponse<R>
    where
        T: Serialize +?Sized,
        R: DeserializeOwned,
    {
        let url = Url::parse(BASE_URL)?.join(endpoint.as_ref())?;
        let response = self
           .client
           .post(url)
           .bearer_auth(&self.api_key)
           .json(&payload)
           .send()
           .await?;

        Ok(response.json().await?)
    }

    pub async fn create_image(&self, request: &ImageRequest) -> ApiResponse<ImageData> {
        self.post(&Endpoint::Image, request).await
    }

    pub async fn completion(&self, request: &CompletionRequest) -> ApiResponse<CompletionsData> {
        self.post(&Endpoint::Completion, request).await
    }
}
