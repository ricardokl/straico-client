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
