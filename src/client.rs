use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

use crate::error::{self, Error};

#[cfg(feature = "file")]
use crate::endpoints::file::FileRequest;
#[cfg(feature = "file")]
use reqwest::multipart::Form;
#[cfg(feature = "file")]
use std::path::Path;

#[cfg(feature = "image")]
use crate::endpoints::image::ImageRequest;

#[cfg(feature = "rag")]
use crate::endpoints::rag::{completion::RagPromptCompletionRequest, create::RagCreateRequest};

#[cfg(feature = "agent")]
use crate::endpoints::agent::{
    completion::AgentCompletionRequest, create::AgentCreateRequest, rag_to_agent::RagToAgentRequest,
};

use crate::endpoints::{completion::completion_request::CompletionRequest, ApiResponseData};

#[cfg(any(feature = "model", feature = "user"))]
use crate::GetEndpoint;

use crate::PostEndpoint;

/// Represents the state where no API key has been set for the request
pub struct NoApiKey;
/// Represents the state where an API key has been set for the request
pub struct ApiKeySet;
/// Represents the state where a payload has been set for the request
pub struct PayloadSet;

/// Builder for making requests to Straico API endpoints
///
/// # Type Parameters
///
/// * `Api` - Represents the authentication state (NoApiKey or ApiKeySet)
/// * `Payload` - Represents the request payload state
/// * `Response` - The expected response type from the API
//pub struct StraicoRequestBuilder<Api, Payload, Response>(
pub struct StraicoRequestBuilder<Api, Payload>(
    RequestBuilder,
    //PhantomData<Response>,
    PhantomData<Payload>,
    PhantomData<Api>,
);

//impl Into<StraicoClient> for Client {
//    /// Converts a reqwest::Client into a StraicoClient
//    ///
//    /// # Returns
//    ///
//    /// A new StraicoClient wrapping the provided reqwest::Client
//    fn into(self) -> StraicoClient {
//        StraicoClient(self)
//    }
//}

const API_BASE_URL: &str = "https://api.straico.com";

/// A client for interacting with the Straico API
///
/// Wraps a reqwest::Client and provides convenient methods for making API requests.
/// Can be created using `StraicoClient::new()` or by converting a reqwest::Client
/// using `Into<StraicoClient>`.
#[derive(Clone)]
pub struct StraicoClient {
    client: Client,
    base_url: String,
}

impl Default for StraicoClient {
    fn default() -> Self {
        Self::new()
    }
}

impl StraicoClient {
    /// Creates a new instance of StraicoClient with default configuration
    ///
    /// This is a convenience constructor that creates a new reqwest::Client with default settings
    /// and a base URL pointing to the production Straico API.
    ///
    /// # Returns
    ///
    /// A new StraicoClient instance ready to make API requests
    pub fn new() -> StraicoClient {
        StraicoClient {
            client: Client::new(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Creates a new instance of StraicoClient with a custom base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the API.
    ///
    /// # Returns
    ///
    /// A new StraicoClient instance configured to use the specified base URL.
    pub fn with_base_url(base_url: String) -> StraicoClient {
        StraicoClient {
            client: Client::new(),
            base_url,
        }
    }

    /// Creates a request builder for the completion endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making completion requests
    pub fn completion<'a>(self) -> StraicoRequestBuilder<NoApiKey, CompletionRequest<'a>> {
        let url = format!("{}{}", self.base_url, PostEndpoint::Completion.as_ref());
        self.client.post(url).into()
    }

    /// Creates a request builder for the image generation endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making image generation requests
    #[cfg(feature = "image")]
    pub fn image(self) -> StraicoRequestBuilder<NoApiKey, ImageRequest> {
        let url = format!("{}{}", self.base_url, PostEndpoint::Image.as_ref());
        self.client.post(url).into()
    }

    /// Creates a request builder for the file upload endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making file upload requests
    #[cfg(feature = "file")]
    pub fn file(self) -> StraicoRequestBuilder<NoApiKey, FileRequest> {
        let url = format!("{}{}", self.base_url, PostEndpoint::File.as_ref());
        self.client.post(url).into()
    }

    /// Creates a request builder for fetching available models
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for retrieving model information
    #[cfg(feature = "model")]
    pub fn models(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!("{}{}", self.base_url, GetEndpoint::Models.as_ref());
        self.client.get(url).into()
    }

    /// Creates a request builder for fetching user information
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for retrieving user data
    #[cfg(feature = "user")]
    pub fn user(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!("{}{}", self.base_url, GetEndpoint::User.as_ref());
        self.client.get(url).into()
    }

    #[cfg(feature = "rag")]
    pub fn create_rag(self) -> StraicoRequestBuilder<NoApiKey, RagCreateRequest> {
        let url = format!("{}{}", self.base_url, PostEndpoint::RagCreate.as_ref());
        self.client.post(url).into()
    }

    #[cfg(feature = "rag")]
    pub fn rag_by_id(self, rag_id: &str) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!(
            "{}{}/{}",
            self.base_url,
            GetEndpoint::RagById.as_ref(),
            rag_id
        );
        self.client.get(url).into()
    }

    #[cfg(feature = "rag")]
    pub fn rag_list(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!("{}{}", self.base_url, GetEndpoint::RagList.as_ref());
        self.client.get(url).into()
    }

    #[cfg(feature = "rag")]
    pub fn rag_prompt_completion(
        self,
        rag_id: &str,
    ) -> StraicoRequestBuilder<NoApiKey, RagPromptCompletionRequest> {
        let url = format!(
            "{}{}/{}/prompt",
            self.base_url,
            PostEndpoint::RagCompletion.as_ref(),
            rag_id
        );
        self.client.post(url).into()
    }

    #[cfg(feature = "agent")]
    pub fn create_agent<'a>(self) -> StraicoRequestBuilder<NoApiKey, AgentCreateRequest<'a>> {
        let url = format!("{}{}", self.base_url, PostEndpoint::AgentCreate.as_ref());
        self.client.post(url).into()
    }

    #[cfg(feature = "agent")]
    pub fn list_agents(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!("{}{}", self.base_url, GetEndpoint::AgentList.as_ref());
        self.client.get(url).into()
    }

    #[cfg(feature = "agent")]
    pub fn agent_details(self, agent_id: &str) -> StraicoRequestBuilder<NoApiKey, PayloadSet> {
        let url = format!(
            "{}{}/{}",
            self.base_url,
            GetEndpoint::AgentDetails.as_ref(),
            agent_id
        );
        self.client.get(url).into()
    }

    #[cfg(feature = "agent")]
    pub fn add_rag_to_agent<'a>(
        self,
        agent_id: &'a str,
    ) -> StraicoRequestBuilder<NoApiKey, RagToAgentRequest<'a>> {
        let url = format!(
            "{}{}/{}/rag",
            self.base_url,
            PostEndpoint::AgentAddRag.as_ref(),
            agent_id
        );
        self.client.post(url).into()
    }

    #[cfg(feature = "agent")]
    pub fn agent_prompt_completion<'a>(
        self,
        agent_id: &'a str,
    ) -> StraicoRequestBuilder<NoApiKey, AgentCompletionRequest<'a>> {
        let url = format!(
            "{}{}/{}/prompt",
            self.base_url,
            PostEndpoint::AgentCompletion.as_ref(),
            agent_id
        );
        self.client.post(url).into()
    }
}

impl<T> StraicoRequestBuilder<NoApiKey, T> {
    /// Sets the Bearer authentication token (API key) for this request
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication. Must implement Display trait.
    ///
    /// # Returns
    ///
    /// A new StraicoRequestBuilder with the ApiKeySet state, preserving the payload and response types
    pub fn bearer_auth<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T> {
        self.0.bearer_auth(api_key).into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "file")]
impl<T> StraicoRequestBuilder<T, FileRequest> {
    /// Creates a multipart form request for file upload
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the file to upload, can be any type implementing AsRef<Path>
    ///
    /// # Returns
    ///
    /// A Result containing a new StraicoRequestBuilder configured with the multipart form,
    /// or an error if file creation fails
    pub async fn multipart<U: AsRef<Path>>(
        self,
        file: U,
    ) -> Result<StraicoRequestBuilder<T, PayloadSet>> {
        let form = Form::new().file("file", file).await?;
        Ok(self.0.multipart(form).into())
    }
}

impl<K, T: Serialize> StraicoRequestBuilder<K, T> {
    /// Sets the JSON payload for the request
    ///
    /// # Arguments
    ///
    /// * `payload` - The payload to serialize as JSON. Must implement Into<T> where T is the expected payload type.
    ///
    /// # Returns
    ///
    /// A new StraicoRequestBuilder with the PayloadSet state, preserving the API key and response types
    pub fn json<U: Into<T>>(self, payload: U) -> StraicoRequestBuilder<K, PayloadSet> {
        self.0.json(&payload.into()).into()
    }
}

impl StraicoRequestBuilder<ApiKeySet, PayloadSet> {
    /// Sends the configured request to the API and deserializes the JSON response
    ///
    /// This method will send the HTTP request that has been configured with authentication
    /// and payload (if applicable), then attempt to parse the response as JSON into
    /// the expected response type.
    ///
    /// # Returns
    ///
    /// A Future that resolves to a Result containing either:
    /// - The deserialized API response data of type `ApiResponseData<V>`
    /// - A reqwest error if the request fails or JSON parsing fails
    pub async fn send(self) -> Result<ApiResponseData> {
        let response = self.0.send().await?;
        let data = response.json().await?;
        Ok(data)
    }
}

impl<T, U> From<RequestBuilder> for StraicoRequestBuilder<T, U> {
    /// Converts a RequestBuilder into a StraicoRequestBuilder
    ///
    /// This implementation allows for easy conversion from reqwest's RequestBuilder
    /// into our typed StraicoRequestBuilder while preserving type information.
    ///
    /// # Arguments
    ///
    /// * `value` - The RequestBuilder to convert
    ///
    /// # Returns
    ///
    /// A new StraicoRequestBuilder wrapping the provided RequestBuilder with appropriate type parameters
    fn from(value: RequestBuilder) -> Self {
        StraicoRequestBuilder(value, PhantomData, PhantomData)
    }
}
