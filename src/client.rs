use core::fmt;
use reqwest::Url;

use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::ApiResponse;
use crate::endpoints::completion::{CompletionRequest, CompletionsData};
use crate::endpoints::file::FileData;
use crate::endpoints::image::{ImageData, ImageRequest};
use crate::endpoints::model::ModelData;
use crate::endpoints::user::UserData;
use crate::{GetEndpoint, PostEndpoint, BASE_URL};

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

    pub async fn post<T, R>(&self, endpoint: &PostEndpoint, payload: &T) -> ApiResponse<R>
    where
        T: Serialize + ?Sized,
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

    pub async fn get<R>(&self, endpoint: &GetEndpoint) -> ApiResponse<R>
    where
        R: DeserializeOwned,
    {
        let url = Url::parse(BASE_URL)?.join(endpoint.as_ref())?;
        let response = self
            .client
            .get(url)
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
