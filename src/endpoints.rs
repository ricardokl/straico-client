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
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseData<T> {
    pub data: T,
    pub success: bool,
}

// Attempt to avoid having to use a generic type parameter for the response data
// Still needs testing
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ApiResponseData2 {
    data: ResponseType,
    success: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(untagged)]
enum ResponseType {
    Completion(completion::completion_response::CompletionData),
    File(file::FileData),
    Image(image::ImageData),
    Model(model::ModelData),
    User(user::UserData),
}
