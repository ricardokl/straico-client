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
use crate::{GetEndpoint, PostEndpoint, BASE_URL};

// Method bearer_auth takes any type that implements Display
pub struct WithKey<K: Display>(K);
pub struct NoKey;

impl From<ReqwestClient> for StraicoClient<NoKey> {
    fn from(client: ReqwestClient) -> Self {
        StraicoClient {
            client,
            api_key: NoKey,
        }
    }
}

trait StraicoClientTrait {
    fn to_straico(self) -> StraicoClient<NoKey>;
}

impl StraicoClientTrait for ReqwestClient {
    fn to_straico(self) -> StraicoClient<NoKey> {
        self.into()
    }
}

pub struct StraicoClient<A> {
    client: ReqwestClient,
    api_key: A,
}

impl<K: Display> Display for WithKey<K> {
    // Passthrough Display
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl StraicoClient<NoKey> {
    pub fn new() -> StraicoClient<NoKey> {
        // Maybe set defaults for client
        ReqwestClient::new().to_straico()
    }

    pub fn set_key<K: Display>(self, key: K) -> StraicoClient<WithKey<K>> {
        // Could also unpack self first and use implicit notation
        StraicoClient {
            client: self.client,
            api_key: WithKey(key),
        }
    }
}

impl<K: Display> StraicoClient<WithKey<K>> {
    pub async fn post<T, R>(&self, endpoint: &PostEndpoint, payload: &T) -> ApiResponse<R>
    where
        T: Serialize +?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", BASE_URL, endpoint.as_ref());
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
