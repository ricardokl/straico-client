// Still need to implement the builder pattern, and add it to client.rs
use serde::{Deserialize, Serialize};

//use std::path::Path;
// Use trait bound "AsRef<Path>"

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChunkingMethod {
    FixedSize,
    Recursive,
    Markdown,
    Python,
    Semantic,
}

#[derive(Serialize, Debug)]
pub struct RagRequest {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_method: Option<ChunkingMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_overlap: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separators: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoint_threshold_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct RagResponse {
    pub success: bool,
    pub data: RagData,
    pub total_coins: f64,
    pub total_words: i64,
}

#[derive(Deserialize, Debug)]
pub struct RagData {
    pub user_id: String,
    pub name: String,
    pub rag_url: String,
    pub original_filename: String,
    pub chunking_method: String,
    pub chunk_size: i64,
    pub chunk_overlap: i64,
    pub breakpoint_threshold_type: String,
    pub separator: String,
    pub separators: Vec<String>,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "__v")]
    pub v: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_rag_response() {
        let json = r#"
        {
            "success": true,
            "data": {
                "user_id": "64ada93f22131d3f5",
                "name": "Rag de prueba txt",
                "rag_url": "https://example.com",
                "original_filename": "sample_txt.txt",
                "chunking_method": "fixed_size",
                "chunk_size": 1000,
                "chunk_overlap": 50,
                "_id": "670565d07e1234eb",
                "createdAt": "2024-10-08T17:03:12.078Z",
                "updatedAt": "2024-10-08T17:03:12.078Z",
                "__v": 0
            },
            "total_coins": 0.97,
            "total_words": 967
        }
        "#;
        let result: RagResponse = serde_json::from_str(json).unwrap();
        assert!(result.success);
        assert_eq!(result.data.user_id, "64ada93f22131d3f5");
        assert_eq!(result.data.name, "Rag de prueba txt");
        assert_eq!(result.data.rag_url, "https://example.com");
        assert_eq!(result.data.original_filename, "sample_txt.txt");
        assert_eq!(result.data.chunking_method, "fixed_size");
        assert_eq!(result.data.chunk_size, 1000);
        assert_eq!(result.data.chunk_overlap, 50);
        assert_eq!(result.data.id, "670565d07e1234eb");
        assert_eq!(result.data.created_at, "2024-10-08T17:03:12.078Z");
        assert_eq!(result.data.updated_at, "2024-10-08T17:03:12.078Z");
        assert_eq!(result.data.v, 0);
        assert_eq!(result.total_coins, 0.97);
        assert_eq!(result.total_words, 967);
    }
}
