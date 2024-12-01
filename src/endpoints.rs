pub mod completion;
pub mod file;
pub mod image;
pub mod model;
pub mod user;

use serde::{Deserialize, Serialize};

/// A generic API response container structure
///
/// # Type Parameters
///
/// * `T` - The type of the data payload contained in the response
///
/// # Fields
///
/// * `data` - The actual data payload of type T returned by the API
/// * `success` - A boolean indicating if the API call was successful
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ApiResponseData<T> {
//     pub data: T,
//     pub success: bool,
// }

/// A container for API response data
///
/// # Fields
///
/// * `data` - The response payload, containing one of several possible response types
/// * `success` - A boolean indicating whether the API call was successful
#[derive(Deserialize, Serialize)]
pub struct ApiResponseData {
    /// The response payload, containing one of several possible response types
    pub data: ResponseType,
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
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseType {
    /// Contains completion data returned from completion API calls
    Completion(completion::completion_response::CompletionData),
    /// Contains file data, skipped during serialization
    #[serde(skip_serializing)]
    File(file::FileData),
    /// Contains image data, skipped during serialization
    #[serde(skip_serializing)]
    Image(image::ImageData),
    /// Contains model data, skipped during serialization
    #[serde(skip_serializing)]
    Model(model::ModelData),
    /// Contains user data, skipped during serialization
    #[serde(skip_serializing)]
    User(user::UserData),
}

impl ResponseType {
    /// Returns the completion data from a ResponseType::Completion variant
    ///
    /// # Returns
    ///
    /// A `completion::completion_response::Completion` containing the completion data
    ///
    /// # Panics
    ///
    /// Will panic if called on any ResponseType variant other than Completion
    pub fn get_completion(self) -> completion::completion_response::Completion {
        match self {
            ResponseType::Completion(data) => data.get_completion(),
            // Method is only used after recieving a completion response from Straico
            _ => unreachable!(),
        }
    }
}
