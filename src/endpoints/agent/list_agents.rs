use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AgentData {
    #[serde(rename = "_id")]
    id: Box<String>,
    uuidv4: Box<String>,
    user_id: Box<String>,
    default_llm: Box<String>,
    custom_prompt: Box<String>,
    name: Box<String>,
    description: Box<String>,
    status: Box<String>,
    tags: Vec<Box<String>>,
    last_interaction: Option<Box<String>>,
    interaction_count: i32,
    visibility: Box<String>,
    #[serde(rename = "createdAt")]
    created_at: Box<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Box<String>,
    #[serde(rename = "__v")]
    v: i32,
    rag_association: Box<String>,
}

#[cfg(test)]
mod tests {
    use super::AgentData;
    use serde_json;

    #[test]
    fn test_agent_data_deserialization() {
        let json_data = r#"{
            "_id": "123",
            "uuidv4": "550e8400-e29b-41d4-a716-446655440000",
            "user_id": "user_123",
            "default_llm": "gpt-4",
            "custom_prompt": "You are helpful",
            "name": "Test Agent",
            "description": "Test Description",
            "status": "active",
            "tags": ["tag1", "tag2"],
            "last_interaction": "2024-01-01T00:00:00Z",
            "interaction_count": 42,
            "visibility": "public",
            "createdAt": "2024-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z",
            "__v": 1,
            "rag_association": "rag_123"
        }"#;

        let agent_data: AgentData =
            serde_json::from_str(json_data).expect("Failed to deserialize AgentData");

        assert_eq!(*agent_data.id, "123");
        assert_eq!(*agent_data.uuidv4, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(*agent_data.user_id, "user_123");
        assert_eq!(*agent_data.default_llm, "gpt-4");
        assert_eq!(*agent_data.custom_prompt, "You are helpful");
        assert_eq!(*agent_data.name, "Test Agent");
        assert_eq!(*agent_data.description, "Test Description");
        assert_eq!(*agent_data.status, "active");
        assert_eq!(
            agent_data.tags,
            vec![Box::from("tag1".to_string()), Box::from("tag2".to_string())]
        );
        assert_eq!(
            *agent_data.last_interaction.unwrap(),
            "2024-01-01T00:00:00Z"
        );
        assert_eq!(agent_data.interaction_count, 42);
        assert_eq!(*agent_data.visibility, "public");
        assert_eq!(*agent_data.created_at, "2024-01-01T00:00:00Z");
        assert_eq!(*agent_data.updated_at, "2024-01-01T00:00:00Z");
        assert_eq!(agent_data.v, 1);
        assert_eq!(*agent_data.rag_association, "rag_123");
    }
}
