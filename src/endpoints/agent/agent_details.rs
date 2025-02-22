use serde::Deserialize;

/// Represents detailed information about a specific agent
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AgentDetailsResponse {
    #[serde(rename = "_id")]
    id: Box<String>,
    uuidv4: Box<String>,
    default_llm: Box<String>,
    custom_prompt: Box<String>,
    name: Box<String>,
    description: Box<String>,
    status: Box<String>,
    tags: Vec<Box<String>>,
    last_interaction: Option<Box<String>>,
    interaction_count: u32,
    visibility: Box<String>,
    #[serde(rename = "createdAt")]
    created_at: Box<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Box<String>,
    #[serde(rename = "__v")]
    version: i32,
    rag_association: Box<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_deserialization() {
        let json_data = r#"
        {
            "_id": "66df87d2bb560c6acabb686s",
            "uuidv4": "c85254c4-0f89-46d7-bf28-428eed764e16",
            "default_llm": "openai/gpt-4o",
            "custom_prompt": "Act a as expert in airline regulations",
            "name": "Expert in airline regulations",
            "description": "Agent expert in airline regulations",
            "status": "active",
            "tags": [],
            "last_interaction": null,
            "interaction_count": 0,
            "visibility": "private",
            "createdAt": "2024-09-09T23:42:10.263Z",
            "updatedAt": "2024-09-10T22:39:46.868Z",
            "__v": 0,
            "rag_association": "https://prompt-rack.s3.amazonaws.com/api/rag/66df87d2bbssscs6acabb6822/index.faiss"
        }
        "#;

        let details: AgentDetailsResponse = serde_json::from_str(json_data).unwrap();

        // Test field deserialization
        assert_eq!(*details.id, "66df87d2bb560c6acabb686s");
        assert_eq!(*details.uuidv4, "c85254c4-0f89-46d7-bf28-428eed764e16");
        assert_eq!(*details.default_llm, "openai/gpt-4o");
        assert_eq!(
            *details.custom_prompt,
            "Act a as expert in airline regulations"
        );
        assert_eq!(*details.name, "Expert in airline regulations");
        assert_eq!(*details.description, "Agent expert in airline regulations");
        assert_eq!(*details.status, "active");
        assert_eq!(details.tags, vec![]);
        assert!(details.last_interaction.is_none());
        assert_eq!(details.interaction_count, 0);
        assert_eq!(*details.visibility, "private");
        assert_eq!(details.interaction_count, 0);
        assert_eq!(details.version, 0);
        assert!(details.last_interaction.is_none());
        assert_eq!(*details.created_at, "2024-09-09T23:42:10.263Z");
        assert_eq!(*details.updated_at, "2024-09-10T22:39:46.868Z");
        assert_eq!(
            *details.rag_association,
            "https://prompt-rack.s3.amazonaws.com/api/rag/66df87d2bbssscs6acabb6822/index.faiss"
        );
    }
}
