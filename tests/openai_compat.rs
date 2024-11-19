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
