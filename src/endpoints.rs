pub mod completion;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "model")]
pub mod model;
#[cfg(feature = "user")]
pub mod user;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A container for API response data
///
/// # Fields
///
/// * `data` - The response payload, containing one of several possible response types (if successful)
/// * `error` - An error message string (if unsuccessful)
/// * `success` - A boolean indicating whether the API call was successful
#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponseData {
    /// The response payload, containing one of several possible response types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResponseType>,
    /// An error message if the request was unsuccessful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// A boolean indicating whether the API call was successful
    success: bool,
}

/// An enum representing different types of API responses
///
/// # Variants
///
/// * `Completion` - Contains completion response data
/// * `File` - Contains file data (skipped during serialization)
/// * `Image` - Contains image data (skipped during serialization)
/// * `Model` - Contains model data (skipped during serialization)
/// * `User` - Contains user data (skipped during serialization)
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    /// Contains completion data returned from completion API calls
    Completion(completion::completion_response::CompletionData),
    /// Contains file data, skipped during serialization
    #[cfg(feature = "file")]
    #[serde(skip_serializing)]
    File(file::FileData),
    /// Contains image data, skipped during serialization
    #[cfg(feature = "image")]
    #[serde(skip_serializing)]
    Image(image::ImageData),
    /// Contains model data, skipped during serialization
    #[cfg(feature = "model")]
    #[serde(skip_serializing)]
    Model(model::ModelData),
    /// Contains user data, skipped during serialization
    #[cfg(feature = "user")]
    #[serde(skip_serializing)]
    User(user::UserData),
}

impl ApiResponseData {
    /// Extracts the completion data from the API response
    ///
    /// # Returns
    ///
    /// * `Ok(Completion)` - The completion data if the API call was successful
    /// * `Err` - The error message if the API call failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The API call failed and returned an error message
    /// * The response data was not of type Completion
    pub fn get_completion(self) -> Result<completion::completion_response::Completion> {
        match self {
            ApiResponseData {
                data: Some(ResponseType::Completion(data)),
                ..
            } => Ok(data.get_completion()),
            ApiResponseData {
                error: Some(err), ..
            } => Err(anyhow::Error::msg(err)),
            _ => unreachable!(),
        }
    }
}
