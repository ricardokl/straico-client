use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AgentCreateRequest<'a> {
    pub name: &'a str,
    pub custom_prompt: &'a str,
    pub default_llm: &'a str,
    pub description: &'a str,
    pub tags: Vec<&'a str>,
}

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_agent_create_request_serialization() {
        let request = AgentCreateRequest {
            name: "TestAgent",
            custom_prompt: "Custom instructions",
            default_llm: "gpt-4",
            description: "Test description",
            tags: vec!["tag1", "tag2"],
        };

        let json = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(
            json,
            r#"{
  "name": "TestAgent",
  "custom_prompt": "Custom instructions",
  "default_llm": "gpt-4",
  "description": "Test description",
  "tags": [
    "tag1",
    "tag2"
  ]
}"#
        );
    }

    #[test]
    fn test_agent_create_response_deserialization() {
        let sample_response = r#"{
            "uuidv4": "123e4567-e89b-12d3-a456-426614174000",
            "user_id": "user_123",
            "default_llm": "gpt-4",
            "custom_prompt": "Custom instructions",
            "name": "TestAgent",
            "description": "Test description",
            "status": "active",
            "tags": ["tag1", "tag2"],
            "last_interaction": "2024-01-01T00:00:00Z",
            "interaction_count": 5,
            "visibility": "private",
            "_id": "507f1f77bcf86cd799439011",
            "createdAt": "2024-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z",
            "__v": 0
        }"#;

        let response: AgentCreateResponse = serde_json::from_str(sample_response).unwrap();

        assert_eq!(
            response.uuidv4.as_ref(),
            "123e4567-e89b-12d3-a456-426614174000"
        );
        assert_eq!(response.user_id.as_ref(), "user_123");
        assert_eq!(response.default_llm.as_ref(), "gpt-4");
        assert_eq!(response.custom_prompt.as_ref(), "Custom instructions");
        assert_eq!(response.name.as_ref(), "TestAgent");
        assert_eq!(response.description.as_ref(), "Test description");
        assert_eq!(response.status.as_ref(), "active");
        assert_eq!(response.tags.len(), 2);
        assert!(response.last_interaction.is_some());
        assert_eq!(response.interaction_count, 5);
        assert_eq!(response.visibility.as_ref(), "private");
        assert_eq!(response.id.as_ref(), "507f1f77bcf86cd799439011");
        assert_eq!(response.created_at.as_ref(), "2024-01-01T00:00:00Z");
        assert_eq!(response.updated_at.as_ref(), "2024-01-01T00:00:00Z");
        assert_eq!(response.v, 0);
    }
}
