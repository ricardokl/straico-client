use serde::Deserialize;
use serde_json::json;
use std::borrow::Cow;
use std::ops::Deref;
use straico::endpoints::completion::completion_request::{CompletionRequest, Prompt};

/// Request format matching OpenAI's chat completion API
#[derive(Deserialize, Clone, Debug)]
#[serde(into = "CompletionRequest")]
pub struct OpenAiRequest<'a> {
    model: Cow<'a, str>,
    messages: Chat<'a>,
    #[serde(alias = "max_completion_tokens")]
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

/// Collection of chat messages in a conversation
#[derive(Deserialize, Clone, Debug)]
pub struct Chat<'a>(pub Vec<Message<'a>>);

impl<'a> Deref for Chat<'a> {
    type Target = Vec<Message<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Chat<'a>> for Prompt<'a> {
    fn from(value: Chat<'a>) -> Self {
        let xml_string = value
            .iter()
            .map(|message| match message.role {
                Role::System => format!("<system>{}</system>", message.content),
                Role::User => format!("<user>{}</user>", message.content),
                Role::Assistant => format!("<assistant>{}</assistant>", message.content),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Prompt::from(Cow::Owned(xml_string))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Message<'a> {
    pub role: Role,
    pub content: Cow<'a, str>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl<'a> From<OpenAiRequest<'a>> for CompletionRequest<'a> {
    fn from(value: OpenAiRequest<'a>) -> Self {
        let builder = CompletionRequest::new()
            .models(value.model)
            .message(value.messages);
        match (value.max_tokens, value.temperature) {
            (Some(x), Some(y)) => builder.max_tokens(x).temperature(y).build(),
            (Some(x), None) => builder.max_tokens(x).build(),
            (None, Some(y)) => builder.temperature(y).build(),
            (None, None) => builder.build(),
        }
    }
}

#[test]
fn test_openai_request_deserialization() {
    let request_json = json!({
        "model": "gpt-4",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Hello!"}
        ],
        "max_tokens": 100,
        "temperature": 0.7
    });

    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    assert_eq!(request.model, "gpt-4");
    assert_eq!(request.max_tokens, Some(100));
}

#[test]
fn test_chat_to_prompt_conversion() {
    let chat = Chat(vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".into(),
        },
        Message {
            role: Role::User,
            content: "Hi!".into(),
        },
    ]);

    let prompt: Prompt = chat.into();
    assert!(prompt.as_ref().contains("<system>"));
    assert!(prompt.as_ref().contains("<user>"));
}

#[test]
fn test_openai_request_parameter_combinations() {
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "max_tokens": 100,
        "temperature": 0.7
    });
    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    let completion_request: CompletionRequest = request.into();
    assert_eq!(completion_request.get_max_tokens(), Some(100));
    assert_eq!(completion_request.get_temperature(), 0.7);

    // Test with only max_tokens
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "max_tokens": 100
    });
    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    let completion_request: CompletionRequest = request.into();
    assert_eq!(completion_request.get_max_tokens(), Some(100));
    assert_eq!(completion_request.get_temperature(), 0.0);

    // Test with only temperature
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "temperature": 0.7
    });
    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    let completion_request: CompletionRequest = request.into();
    assert_eq!(completion_request.get_max_tokens(), None);
    assert_eq!(completion_request.get_temperature(), 0.7);
}

#[test]
fn test_complex_chat_conversion() {
    let chat = Chat(vec![
        Message {
            role: Role::System,
            content: "System prompt".into(),
        },
        Message {
            role: Role::User,
            content: "User message 1".into(),
        },
        Message {
            role: Role::Assistant,
            content: "Assistant response".into(),
        },
        Message {
            role: Role::User,
            content: "User message 2".into(),
        },
    ]);

    let prompt: Prompt = chat.into();
    let prompt_str = prompt.as_ref();
    assert!(prompt_str.contains("<system>System prompt</system>"));
    assert!(prompt_str.contains("<user>User message 1</user>"));
    assert!(prompt_str.contains("<assistant>Assistant response</assistant>"));
    assert!(prompt_str.contains("<user>User message 2</user>"));
}

#[test]
fn test_request_parsing_errors() {
    // Test invalid role
    let request_json = json!({
        "model": "gpt-4",
        "messages": [
            {"role": "invalid_role", "content": "Hello!"}
        ]
    });
    let result: Result<OpenAiRequest, _> = serde_json::from_value(request_json);
    assert!(result.is_err());

    // Test missing required fields
    let request_json = json!({
        "messages": [
            {"role": "user", "content": "Hello!"}
        ]
    });
    let result: Result<OpenAiRequest, _> = serde_json::from_value(request_json);
    assert!(result.is_err());

    // Test invalid temperature
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "temperature": "invalid"
    });
    let result: Result<OpenAiRequest, _> = serde_json::from_value(request_json);
    assert!(result.is_err());
}
