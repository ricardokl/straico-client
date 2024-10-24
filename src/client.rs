use core::fmt::{Display, Formatter, Result};
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};

use crate::common::ApiResponse;
use crate::endpoints::{
    audio::{TranscriptionRequest, TranscriptionData},
    completion::{CompletionRequest, CompletionsData},
    embeddings::{EmbeddingsRequest, EmbeddingsData},
    file::FileData,
    image::{ImageData, ImageRequest},
    model::ModelData,
    user::UserData,
};
use crate::{GetEndpoint, PostEndpoint};

pub trait IntoStraicoClient {
    fn to_straico(self, api_key: String) -> StraicoClient;
}

impl IntoStraicoClient for ReqwestClient {
    fn to_straico(self, api_key: String) -> StraicoClient {
        StraicoClient {
            client: self,
            api_key,
        }
    }
}

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

    pub async fn post_multipart<R>(&self, endpoint: &PostEndpoint, form: Form) -> ApiResponse<R>
    where
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(endpoint.as_ref())
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn upload_file(&self, request: &FileUploadRequest) -> ApiResponse<FileData> {
        let form = request.to_form().await?;
        self.post_multipart(&PostEndpoint::File, form).await
    }

    pub async fn transcribe(&self, request: &TranscriptionRequest) -> ApiResponse<TranscriptionData> {
        self.post(&PostEndpoint::AudioTranscription, request).await
    }

    pub async fn generate_audio(&self, request: &GenerationRequest) -> ApiResponse<GenerationData> {
        self.post(&PostEndpoint::AudioGeneration, request).await
    }

    pub async fn embeddings(&self, request: &EmbeddingsRequest) -> ApiResponse<EmbeddingsData> {
        self.post(&PostEndpoint::Embeddings, request).await
    }
}
