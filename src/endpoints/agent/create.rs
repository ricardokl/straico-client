use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AgentCreateRequest<'a> {
    name: &'a str,
    custom_prompt: &'a str,
    default_llm: &'a str,
    description: &'a str,
    tags: Vec<&'a str>,
}

#[derive(Default)]
pub struct NotSet;
pub struct Set<T>(T);

#[derive(Default)]
pub struct AgentCreateRequestBuilder<T = NotSet, V = NotSet, U = NotSet, R = NotSet, S = NotSet> {
    name: T,
    custom_prompt: V,
    default_llm: U,
    description: R,
    tags: S,
}

impl AgentCreateRequest<'_> {
    pub fn new() -> AgentCreateRequestBuilder {
        AgentCreateRequestBuilder::default()
    }
}

impl<T, U, V, R> AgentCreateRequestBuilder<NotSet, T, U, V, R> {
    pub fn name<'a>(self, name: &'a str) -> AgentCreateRequestBuilder<Set<&'a str>, T, U, V, R> {
        AgentCreateRequestBuilder {
            name: Set(name),
            custom_prompt: self.custom_prompt,
            default_llm: self.default_llm,
            description: self.description,
            tags: self.tags,
        }
    }
}

impl<T, U, V, R> AgentCreateRequestBuilder<T, NotSet, U, V, R> {
    pub fn custom_prompt<'a>(
        self,
        custom_prompt: &'a str,
    ) -> AgentCreateRequestBuilder<T, Set<&'a str>, U, V, R> {
        AgentCreateRequestBuilder {
            name: self.name,
            custom_prompt: Set(custom_prompt),
            default_llm: self.default_llm,
            description: self.description,
            tags: self.tags,
        }
    }
}

impl<T, U, V, R> AgentCreateRequestBuilder<T, U, NotSet, V, R> {
    pub fn default_llm<'a>(
        self,
        default_llm: &'a str,
    ) -> AgentCreateRequestBuilder<T, U, Set<&'a str>, V, R> {
        AgentCreateRequestBuilder {
            name: self.name,
            custom_prompt: self.custom_prompt,
            default_llm: Set(default_llm),
            description: self.description,
            tags: self.tags,
        }
    }
}

impl<T, U, V, R> AgentCreateRequestBuilder<T, U, V, NotSet, R> {
    pub fn description<'a>(
        self,
        description: &'a str,
    ) -> AgentCreateRequestBuilder<T, U, V, Set<&'a str>, R> {
        AgentCreateRequestBuilder {
            name: self.name,
            custom_prompt: self.custom_prompt,
            default_llm: self.default_llm,
            description: Set(description),
            tags: self.tags,
        }
    }
}

impl<T, U, V, R> AgentCreateRequestBuilder<T, U, V, R, NotSet> {
    pub fn tags<'a>(
        self,
        tags: Vec<&'a str>,
    ) -> AgentCreateRequestBuilder<T, U, V, R, Set<Vec<&'a str>>> {
        AgentCreateRequestBuilder {
            name: self.name,
            custom_prompt: self.custom_prompt,
            default_llm: self.default_llm,
            description: self.description,
            tags: Set(tags),
        }
    }
}

impl<'a>
    AgentCreateRequestBuilder<
        Set<&'a str>,
        Set<&'a str>,
        Set<&'a str>,
        Set<&'a str>,
        Set<Vec<&'a str>>,
    >
{
    pub fn build(self) -> AgentCreateRequest<'a> {
        AgentCreateRequest {
            name: (self.name).0,
            custom_prompt: (self.custom_prompt).0,
            default_llm: (self.default_llm).0,
            description: (self.description).0,
            tags: (self.tags).0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AgentCreateResponse {
    uuidv4: Box<String>,
    user_id: Box<String>,
    default_llm: Box<String>,
    custom_prompt: Box<String>,
    name: Box<String>,
    description: Box<String>,
    status: Box<String>,
    tags: Vec<Box<str>>,
    last_interaction: Option<Box<String>>,
    interaction_count: u32,
    visibility: Box<String>,
    #[serde(rename = "_id")]
    id: Box<String>,
    #[serde(rename = "createdAt")]
    created_at: Box<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Box<String>,
    #[serde(rename = "__v")]
    v: u32,
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
