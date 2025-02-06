#[cfg(feature = "agent")]
pub mod agent;
pub mod completion;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "model")]
pub mod model;
#[cfg(feature = "rag")]
pub mod rag;
#[cfg(feature = "user")]
pub mod user;

use anyhow::Result;
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
    Error {
        error: String,
    },
    Message {
        message: String,
    },
    Data {
        data: ResponseType,
    },
    DetailedData {
        data: ResponseType,
        total_words: f64,
        total_coins: f64,
    },
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
    #[cfg(feature = "rag")]
    #[serde(skip_serializing)]
    RagCreate(rag::create::RagData),
    #[cfg(feature = "rag")]
    #[serde(skip_serializing)]
    RagCompletion(rag::completion::RagPromptCompletionData),
    #[cfg(feature = "rag")]
    #[serde(skip_serializing)]
    RagById(rag::rag_by_id::RagByIdData),
    #[cfg(feature = "rag")]
    #[serde(skip_serializing)]
    RagList(rag::list::RagListResponseData),
    #[cfg(feature = "agent")]
    #[serde(skip_serializing)]
    AgentCreate(agent::create::AgentCreateResponse),
    #[sfg(feature = "agent")]
    #[serde(skip_serializing)]
    AgentCompletion(agent::completion::AgentCompletionResponse),
    #[sfg(feature = "agent")]
    #[serde(skip_serializing)]
    AgentList(agent::list_agents::AgentData),
    #[sfg(feature = "agent")]
    #[serde(skip_serializing)]
    AgentDetails(agent::agent_details::AgentDetailsResponse),
    #[sfg(feature = "agent")]
    #[serde(skip_serializing)]
    AgentAddRag(agent::rag_to_agent::RagToAgentResponse),
}

impl ApiResponseData {
    pub fn get_completion(self) -> Result<completion::completion_response::Completion> {
        match self.response {
            ApiResponseVariant::Data {
                data: ResponseType::Completion(data),
            } => Ok(data.get_completion_data()),
            ApiResponseVariant::DetailedData {
                data: ResponseType::Completion(data),
                ..
            } => Ok(data.get_completion_data()),
            ApiResponseVariant::Error { error } => Err(anyhow::Error::msg(error)),
            _ => Err(anyhow::Error::msg("Invalid response type for completion")),
        }
    }
}
