use crate::endpoints::ApiResponseData;
use actix_web::error::InternalError;
use actix_web::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

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

impl From<ApiError> for actix_web::Error {
    fn from(error: ApiError) -> Self {
        match error {
            // ApiError::Network(error) => actix_web::Error::from(error),
            ApiError::Parsing(error) => Error::from(error),
            _ => InternalError::new(
                "Reqwest Error",
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into(),
        }
    }
}
