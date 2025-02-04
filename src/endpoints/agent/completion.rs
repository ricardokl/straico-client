use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Similarity,
    Mmr,
    SimilarityScoreThreshold,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCompletionRequest<'a> {
    pub prompt: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<SearchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_mult: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCompletionResponse {
    pub answer: Box<String>,
    pub references: Vec<Reference>,
    pub file_name: Box<String>,
    pub coins_used: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub page_content: Box<String>,
    pub page: u32,
}
