use serde_json::json;
use straico::endpoints::completion::completion_request::Prompt;
use straico_server::{OpenAiRequest, Chat, Message, Role};

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
            content: "You are a helpful assistant.".into()
        },
        Message {
            role: Role::User,
            content: "Hi!".into()
        }
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
    assert_eq!(completion_request.max_tokens(), Some(100));
    assert_eq!(completion_request.temperature(), Some(0.7));

    // Test with only max_tokens
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "max_tokens": 100
    });
    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    let completion_request: CompletionRequest = request.into();
    assert_eq!(completion_request.max_tokens(), Some(100));
    assert_eq!(completion_request.temperature(), None);

    // Test with only temperature
    let request_json = json!({
        "model": "gpt-4",
        "messages": [],
        "temperature": 0.7
    });
    let request: OpenAiRequest = serde_json::from_value(request_json).unwrap();
    let completion_request: CompletionRequest = request.into();
    assert_eq!(completion_request.max_tokens(), None);
    assert_eq!(completion_request.temperature(), Some(0.7));
}

#[test]
fn test_complex_chat_conversion() {
    let chat = Chat(vec![
        Message {
            role: Role::System,
            content: "System prompt".into()
        },
        Message {
            role: Role::User,
            content: "User message 1".into()
        },
        Message {
            role: Role::Assistant,
            content: "Assistant response".into()
        },
        Message {
            role: Role::User,
            content: "User message 2".into()
        }
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
