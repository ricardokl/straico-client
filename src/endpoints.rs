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
    pub success: bool,
    #[serde(flatten)]
    pub response: ApiResponseVariant,
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
    Rag(RagResponseType),
    #[cfg(feature = "agent")]
    #[serde(skip_serializing)]
    Agent(AgentResponseType),
}

// TODO: Check If Update needs an extra variant
#[cfg(feature = "rag")]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RagResponseType {
    RagCreate(rag::create::RagData),
    RagCompletion(rag::completion::RagPromptCompletionData),
    RagById(rag::rag_by_id::RagByIdData),
    RagList(rag::list::RagListResponseData),
}

// TODO: Check if Update Agent needs an extra variant
#[cfg(feature = "agent")]
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AgentResponseType {
    AgentCreate(agent::create::AgentCreateResponse),
    AgentCompletion(agent::completion::AgentCompletionResponse),
    AgentList(agent::list_agents::AgentData),
    AgentDetails(agent::agent_details::AgentDetailsResponse),
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
