use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RagToAgentRequest<'a> {
    pub rag: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RagToAgentResponse {
    pub uuidv4: Box<String>,
    pub user_id: Box<String>,
    pub default_llm: Box<String>,
    pub custom_prompt: Box<String>,
    pub name: Box<String>,
    pub description: Box<String>,
    pub status: Box<String>,
    pub tags: Vec<Box<String>>,
    pub last_interaction: Option<Box<String>>,
    pub interaction_count: u32,
    pub visibility: Box<String>,
    #[serde(rename = "_id")]
    pub id: Box<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Box<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Box<String>,
    #[serde(rename = "__v")]
    pub v: u32,
}
