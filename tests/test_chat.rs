use straico_client::chat::{Chat, Tool};
use straico_client::endpoints::completion::completion_response::{Content, Message};
use serde_json::json;

#[test]
fn test_chat_to_prompt_default_format_user_only() {
    let chat = Chat(vec![Message::User {
        content: Content::Text("Hello".into()),
    }]);
    let prompt = chat.to_prompt(None, "some_other_model");
    let expected = "<system>You are a helpful assistant.\n</system><user>Hello</user>";
    assert_eq!(prompt.as_ref(), expected);
}

#[test]
fn test_chat_to_prompt_anthropic_format() {
    let chat = Chat(vec![Message::User {
        content: Content::Text("Hello".into()),
    }]);
    let prompt = chat.to_prompt(None, "anthropic/claude-3");
    let expected = "You are a helpful assistant.\n\n\nHuman: Hello\n";
    assert_eq!(prompt.as_ref(), expected);
}

#[test]
fn test_chat_to_prompt_llama3_format() {
    let chat = Chat(vec![Message::User {
        content: Content::Text("Hello".into()),
    }]);
    let prompt = chat.to_prompt(None, "meta/llama3");
    let expected = "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\nYou are a helpful assistant.\n<|eot_id|><|start_header_id|>user<|end_header_id|>\n\nHello<|eot_id|>";
    assert_eq!(prompt.as_ref(), expected);
}

#[test]
fn test_chat_to_prompt_with_system_message() {
    let chat = Chat(vec![
        Message::System {
            content: Content::Text("You are a pirate.".into()),
        },
        Message::User {
            content: Content::Text("Hello".into()),
        },
    ]);
    let prompt = chat.to_prompt(None, "default");
    let expected = "<system>You are a pirate.\n</system><user>Hello</user>";
    assert_eq!(prompt.as_ref(), expected);
}

#[test]
fn test_chat_to_prompt_with_tools() {
    let chat = Chat(vec![Message::User {
        content: Content::Text("What is the weather in Boston?".into()),
    }]);
    let tools = Some(vec![Tool::Function {
        name: "get_weather".to_string(),
        description: Some("gets the weather".to_string()),
        parameters: Some(json!({"location": "string"})),
    }]);
    let prompt = chat.to_prompt(tools, "default");

    // The exact string is complex, so we'll just check for substrings
    assert!(prompt.as_ref().contains("<system>"));
    assert!(prompt.as_ref().contains("You are a helpful assistant."));
    assert!(prompt.as_ref().contains("# Tools"));
    assert!(prompt.as_ref().contains("get_weather"));
    assert!(prompt.as_ref().contains("</system>"));
    assert!(prompt.as_ref().contains("<user>"));
    assert!(prompt.as_ref().contains("What is the weather in Boston?"));
    assert!(prompt.as_ref().contains("</user>"));
}
