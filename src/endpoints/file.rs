use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Data {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct GetApiKeysByDateRequest {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct GetApiKeysByDateResponse {
    pub api_keys: Vec<ApiKey>,
}
