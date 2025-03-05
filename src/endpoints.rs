pub mod completion;

use anyhow::Result;
use completion::completion_response::CompletionData;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponseData {
    success: bool,
    #[serde(flatten)]
    response: ApiResponseVariant,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ApiResponseVariant {
    Error { error: String },
    Data { data: CompletionData },
}

impl ApiResponseData {
    pub fn get_completion(self) -> Result<completion::completion_response::Completion> {
        match self.response {
            ApiResponseVariant::Data { data } => Ok(data.get_completion_data()),
            ApiResponseVariant::Error { error } => Err(anyhow::Error::msg(error)),
        }
    }
}
