use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Similarity,
    Mmr,
    SimilarityScoreThreshold,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RagPromptCompletionRequest {
    // Required fields
    pub prompt: String,
    pub model: String,
    // Optional parameters for the underlying retriever (FAISS based VectorStoreRetriever)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<SearchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_mult: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

// A single reference returned as part of the response.
#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    pub page_content: String,
    pub page: i32,
}

// Data object returned within the response.
#[derive(Serialize, Deserialize, Debug)]
pub struct RagPromptCompletionData {
    pub answer: String,
    pub references: Vec<Reference>,
    pub file_name: String,
    pub coins_used: f64,
}
