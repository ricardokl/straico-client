use core::fmt::{Display, Formatter, Result};
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};

use crate::common::ApiResponse;
use crate::endpoints::{
    completion::{CompletionRequest, CompletionsData},
    file::FileData,
    image::{ImageData, ImageRequest},
    model::ModelData,
    user::UserData,
};
use crate::{GetEndpoint, PostEndpoint};

pub struct StraicoClient {
    client: ReqwestClient,
    api_key: String,
}

impl StraicoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: ReqwestClient::new(),
            api_key,
        }
    }

    pub async fn post<T, R>(&self, endpoint: &PostEndpoint, payload: &T) -> ApiResponse<R>
    where
        T: Serialize +?Sized,
        R: DeserializeOwned,
    {
        let response = self
           .client
           .post(endpoint.as_ref())
           .bearer_auth(&self.api_key)
           .json(&payload)
           .send()
           .await?;

        Ok(response.json().await?)
    }

    pub async fn get<R>(&self, endpoint: &GetEndpoint) -> ApiResponse<R>
    where
        R: DeserializeOwned,
    {
        let response = self
           .client
           .get(endpoint.as_ref())
           .bearer_auth(&self.api_key)
           .send()
           .await?;

        Ok(response.json().await?)
    }

    pub async fn create_image(&self, request: &ImageRequest) -> ApiResponse<ImageData> {
        self.post(&PostEndpoint::Image, request).await
    }

    pub async fn completion(&self, request: &CompletionRequest) -> ApiResponse<CompletionsData> {
        self.post(&PostEndpoint::Completion, request).await
    }

    pub async fn user(&self) -> ApiResponse<UserData> {
        self.get(&GetEndpoint::User).await
    }

    pub async fn models(&self) -> ApiResponse<ModelData> {
        self.get(&GetEndpoint::Models).await
    }

    pub async fn file(&self) -> ApiResponse<FileData> {
        // Needs to implement file upload
        todo!()
    }
}
