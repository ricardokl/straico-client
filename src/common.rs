use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub success: bool,
}
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(ReqwestError),
    Parsing(SerdeError),
}

impl From<ReqwestError> for ApiError {
    fn from(error: ReqwestError) -> Self {
        ApiError::Network(error)
    }
}

impl From<SerdeError> for ApiError {
    fn from(error: SerdeError) -> Self {
        ApiError::Parsing(error)
    }
}
