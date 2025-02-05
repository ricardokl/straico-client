use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RagToAgentRequest<'a> {
    rag: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RagToAgentResponse {
    uuidv4: Box<String>,
    user_id: Box<String>,
    default_llm: Box<String>,
    custom_prompt: Box<String>,
    name: Box<String>,
    description: Box<String>,
    status: Box<String>,
    tags: Vec<Box<String>>,
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
    fn test_rag_to_agent_response_deserialization() {
        let json = r#"{
            "uuidv4": "test_uuid",
            "user_id": "test_user",
            "default_llm": "test_llm",
            "custom_prompt": "test_prompt",
            "name": "test_name",
            "description": "test_description",
            "status": "test_status",
            "tags": ["tag1", "tag2"],
            "last_interaction": "test_interaction",
            "interaction_count": 1,
            "visibility": "test_visibility",
            "_id": "test_id",
            "createdAt": "test_created",
            "updatedAt": "test_updated",
            "__v": 1
        }"#;
        let response: RagToAgentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(*response.uuidv4, "test_uuid");
        assert_eq!(*response.user_id, "test_user");
        assert_eq!(*response.default_llm, "test_llm");
        assert_eq!(*response.custom_prompt, "test_prompt");
        assert_eq!(*response.name, "test_name");
        assert_eq!(*response.description, "test_description");
        assert_eq!(*response.status, "test_status");
        assert_eq!(response.tags.len(), 2);
        assert_eq!(*response.last_interaction.unwrap(), "test_interaction");
        assert_eq!(response.interaction_count, 1);
        assert_eq!(*response.visibility, "test_visibility");
        assert_eq!(*response.id, "test_id");
        assert_eq!(*response.created_at, "test_created");
        assert_eq!(*response.updated_at, "test_updated");
        assert_eq!(response.v, 1);
    }
}
