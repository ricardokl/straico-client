use futures::TryFutureExt;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, future::Future, marker::PhantomData};

#[cfg(feature = "file")]
use crate::endpoints::file::{FileData, FileRequest};
#[cfg(feature = "file")]
use reqwest::multipart::Form;
#[cfg(feature = "file")]
use std::path::Path;

#[cfg(feature = "image")]
use crate::endpoints::image::{ImageData, ImageRequest};

#[cfg(feature = "model")]
use crate::endpoints::model::ModelData;

#[cfg(feature = "user")]
use crate::endpoints::user::UserData;

#[cfg(feature = "rag")]
use crate::endpoints::rag::create::{RagData, RagRequest};

use crate::endpoints::{
    completion::completion_request::CompletionRequest,
    completion::completion_response::CompletionData, ApiResponseData,
};

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
pub struct StraicoRequestBuilder<Api, Payload, Response>(
    RequestBuilder,
    PhantomData<Response>,
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

impl From<Client> for StraicoClient {
    fn from(value: Client) -> Self {
        StraicoClient(value)
    }
}

/// A client for interacting with the Straico API
///
/// Wraps a reqwest::Client and provides convenient methods for making API requests.
/// Can be created using `StraicoClient::new()` or by converting a reqwest::Client
/// using `Into<StraicoClient>`.
#[derive(Clone, Default)]
pub struct StraicoClient(Client);

impl StraicoClient {
    /// Creates a new instance of StraicoClient with default configuration
    ///
    /// This is a convenience constructor that creates a new reqwest::Client with default settings
    /// and converts it into a StraicoClient.
    ///
    /// # Returns
    ///
    /// A new StraicoClient instance ready to make API requests
    pub fn new() -> StraicoClient {
        StraicoClient::default()
    }

    /// Creates a request builder for the completion endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making completion requests
    pub fn completion<'a>(
        self,
    ) -> StraicoRequestBuilder<NoApiKey, CompletionRequest<'a>, CompletionData> {
        self.0.post(PostEndpoint::Completion.as_ref()).into()
    }

    /// Creates a request builder for the image generation endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making image generation requests
    #[cfg(feature = "image")]
    pub fn image(self) -> StraicoRequestBuilder<NoApiKey, ImageRequest, ImageData> {
        self.0.post(PostEndpoint::Image.as_ref()).into()
    }

    /// Creates a request builder for the file upload endpoint
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for making file upload requests
    #[cfg(feature = "file")]
    pub fn file(self) -> StraicoRequestBuilder<NoApiKey, FileRequest, FileData> {
        self.0.post(PostEndpoint::File.as_ref()).into()
    }

    /// Creates a request builder for fetching available models
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for retrieving model information
    #[cfg(feature = "model")]
    pub fn models(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, ModelData> {
        self.0.get(GetEndpoint::Models.as_ref()).into()
    }

    /// Creates a request builder for fetching user information
    ///
    /// # Returns
    ///
    /// A `StraicoRequestBuilder` configured for retrieving user data
    #[cfg(feature = "user")]
    pub fn user(self) -> StraicoRequestBuilder<NoApiKey, PayloadSet, UserData> {
        self.0.get(GetEndpoint::User.as_ref()).into()
    }

    #[cfg(feature = "rag")]
    pub fn create_rag(self) -> StraicoRequestBuilder<NoApiKey, RagRequest, RagData> {
        todo!()
    }
}

impl<T, U> StraicoRequestBuilder<NoApiKey, T, U> {
    /// Sets the Bearer authentication token (API key) for this request
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for authentication. Must implement Display trait.
    ///
    /// # Returns
    ///
    /// A new StraicoRequestBuilder with the ApiKeySet state, preserving the payload and response types
    pub fn bearer_auth<K: Display>(self, api_key: K) -> StraicoRequestBuilder<ApiKeySet, T, U> {
        self.0.bearer_auth(api_key).into()
    }
}

#[cfg(feature = "file")]
type FormResult<T> = anyhow::Result<StraicoRequestBuilder<T, PayloadSet, FileData>>;
#[cfg(feature = "file")]
impl<T> StraicoRequestBuilder<T, FileRequest, FileData> {
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
    pub async fn multipart<U: AsRef<Path>>(self, file: U) -> FormResult<T> {
        let form = Form::new().file("file", file).await?;
        Ok(self.0.multipart(form).into())
    }
}

impl<K, T: Serialize, V> StraicoRequestBuilder<K, T, V> {
    /// Sets the JSON payload for the request
    ///
    /// # Arguments
    ///
    /// * `payload` - The payload to serialize as JSON. Must implement Into<T> where T is the expected payload type.
    ///
    /// # Returns
    ///
    /// A new StraicoRequestBuilder with the PayloadSet state, preserving the API key and response types
    pub fn json<U: Into<T>>(self, payload: U) -> StraicoRequestBuilder<K, PayloadSet, V> {
        self.0.json(&payload.into()).into()
    }
}

impl<V: for<'a> Deserialize<'a>> StraicoRequestBuilder<ApiKeySet, PayloadSet, V> {
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
    pub fn send(self) -> impl Future<Output = reqwest::Result<ApiResponseData>> {
        self.0.send().and_then(Response::json)
    }
}

impl<T, U, V> From<RequestBuilder> for StraicoRequestBuilder<T, U, V> {
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
        StraicoRequestBuilder(value, PhantomData, PhantomData, PhantomData)
    }
}
