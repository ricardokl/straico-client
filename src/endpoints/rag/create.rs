use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarkdownRag {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_overlap: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PythonRag {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_overlap: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FixedSizeRag {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_overlap: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RecursiveRag {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_overlap: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separators: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointThresholdType {
    Percentile,
    Interquartile,
    StandardDeviation,
    Gradient,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SemanticRag {
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    breakpoint_threshold_type: Option<BreakpointThresholdType>,
}

#[derive(Serialize, Debug)]
pub struct RagCreateRequest {
    name: String,
    description: String,
    files: Vec<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    chunking_method: Option<ChunkingMethod>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "chunking_method", rename_all = "snake_case")]
pub enum ChunkingMethod {
    Markdown(MarkdownRag),
    Python(PythonRag),
    FixedSize(FixedSizeRag),
    Recursive(RecursiveRag),
    Semantic(SemanticRag),
}

#[derive(Deserialize, Debug)]
pub struct RagData {
    pub user_id: Box<str>,
    pub name: Box<str>,
    pub rag_url: Box<str>,
    pub original_filename: Box<str>,
    #[serde(flatten)]
    pub chunking_method: ChunkingMethod,
    #[serde(rename = "_id")]
    pub id: Box<str>,
    #[serde(rename = "createdAt")]
    pub created_at: Box<str>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Box<str>,
    #[serde(rename = "__v")]
    pub v: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_rag_response() {
        let json = r#"
        {
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
        }
        "#;
        let result: RagData = serde_json::from_str(json).expect("Deserialization failed");
        assert_eq!(result.user_id, "64ada93f22131d3f5".into());
        assert_eq!(result.name, "Rag de prueba txt".into());
        assert_eq!(result.rag_url, "https://example.com".into());
        assert_eq!(result.original_filename, "sample_txt.txt".into());
        assert_eq!(
            result.chunking_method,
            ChunkingMethod::FixedSize(FixedSizeRag {
                chunk_size: Some(1000),
                chunk_overlap: Some(50),
                separator: None
            })
        );
        assert_eq!(result.id, "670565d07e1234eb".into());
        assert_eq!(result.created_at, "2024-10-08T17:03:12.078Z".into());
        assert_eq!(result.updated_at, "2024-10-08T17:03:12.078Z".into());
        assert_eq!(result.v, 0);
    }
}
