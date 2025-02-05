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
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn test_search_type_serialization() {
        assert_eq!(
            serde_json::to_string(&SearchType::Similarity).unwrap(),
            r#""similarity""#
        );
        assert_eq!(
            serde_json::to_string(&SearchType::Mmr).unwrap(),
            r#""mmr""#
        );
        assert_eq!(
            serde_json::to_string(&SearchType::SimilarityScoreThreshold).unwrap(),
            r#""similarity_score_threshold""#
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = AgentCompletionRequest {
            prompt: "test prompt",
            search_type: Some(SearchType::Mmr),
            k: Some(5),
            fetch_k: Some(20),
            lambda_mult: Some(0.5),
            score_threshold: Some(0.8),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["prompt"], "test prompt");
        assert_eq!(json["search_type"], "mmr");
        assert_eq!(json["k"], 5);
        assert_eq!(json["fetch_k"], 20);
        assert_eq!(json["lambda_mult"], 0.5);
        assert_eq!(json["score_threshold"], 0.8);
    }

    #[test]
    fn test_request_serialization_omits_none() {
        let request = AgentCompletionRequest {
            prompt: "partial request",
            search_type: None,
            k: None,
            fetch_k: None,
            lambda_mult: None,
            score_threshold: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["prompt"], "partial request");
        assert!(json.get("search_type").is_none());
        assert!(json.get("k").is_none());
        assert!(json.get("fetch_k").is_none());
        assert!(json.get("lambda_mult").is_none());
        assert!(json.get("score_threshold").is_none());
    }

    #[test]
    fn test_response_deserialization() {
        let json = json!({
            "answer": "42",
            "references": [{
                "page_content": "content",
                "page": 1
            }],
            "file_name": "test.txt",
            "coins_used": 3.14
        });

        let response: AgentCompletionResponse = serde_json::from_value(json).unwrap();
        assert_eq!(*response.answer, "42");
        assert_eq!(response.references.len(), 1);
        assert_eq!(*response.references[0].page_content, "content");
        assert_eq!(response.references[0].page, 1);
        assert_eq!(*response.file_name, "test.txt");
        assert_eq!(response.coins_used, 3.14);
    }

    #[test]
    fn test_reference_roundtrip() {
        let reference = Reference {
            page_content: Box::new("text".into()),
            page: 42,
        };

        let json = serde_json::to_string(&reference).unwrap();
        let deserialized: Reference = serde_json::from_str(&json).unwrap();
        
        assert_eq!(*deserialized.page_content, "text");
        assert_eq!(deserialized.page, 42);
    }
}
