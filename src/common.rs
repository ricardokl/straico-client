use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Error as SerdeError;

#[derive(Debug, Deserialize)]
pub struct ApiResponseData<T> {
    pub data: T,
    pub success: bool,
}

pub type ApiResponse<T> = Result<ApiResponseData<T>, ApiError>;

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
