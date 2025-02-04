use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreateRequest<'a> {
    pub name: &'a str,
    pub custom_prompt: &'a str,
    pub default_llm: &'a str,
    pub description: &'a str,
    pub tags: Vec<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreateResponse {
    pub uuidv4: Box<String>,
    pub user_id: Box<String>,
    pub default_llm: Box<String>,
    pub custom_prompt: Box<String>,
    pub name: Box<String>,
    pub description: Box<String>,
    pub status: Box<String>,
    pub tags: Vec<Box<str>>,
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
