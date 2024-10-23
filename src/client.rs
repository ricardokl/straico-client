use core::fmt::{Display, Formatter, Result};
use reqwest::Url;

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
use crate::{GetEndpoint, PostEndpoint, BASE_URL};

pub struct Client<A> {
    client: ReqwestClient,
    api_key: A,
}

impl From<ReqwestClient> for Client<NoKey> {
    fn from(client: ReqwestClient) -> Self {
        Client {
            client,
            api_key: NoKey,
        }
    }
}

// Method bearer_auth takes any type that implements Display
pub struct WithKey<K: Display>(K);
pub struct NoKey;

impl<K: Display> Display for WithKey<K> {
    // Passthrough Display
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl Client<NoKey> {
    pub fn new() -> Client<NoKey> {
        // Maybe set defaults for client
        Self {
            client: ReqwestClient::new(),
            api_key: NoKey,
        }
    }

    pub fn set_key<K>(self, key: K) -> Client<WithKey<K>>
    where
        K: Display,
    {
        // Could also unpack self first and use implicit notation
        Client {
            client: self.client,
            api_key: WithKey::<K>(key),
        }
    }
}

impl<K: Display> Client<WithKey<K>> {
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
