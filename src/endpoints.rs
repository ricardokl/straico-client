pub mod completion;
pub mod file;
pub mod image;
pub mod model;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseData<T> {
    pub data: T,
    pub success: bool,
}

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
