use straico_client::endpoints::completion::completion_response::{
    Choice, Completion, Content, Message, Usage,
};

fn create_test_completion(message: Message, finish_reason: &str) -> Completion {
    Completion {
        id: "cmpl-123".into(),
        object: "chat.completion".into(),
        created: 1677652288,
        model: "gpt-3.5-turbo-0125".into(),
        choices: vec![Choice {
            index: 0,
            message,
            finish_reason: finish_reason.into(),
        }],
        usage: Usage {
            prompt_tokens: 9,
            completion_tokens: 19,
            total_tokens: 28,
        },
    }
}

#[test]
fn test_completion_parse_tool_calls() {
    let message = Message::Assistant {
        content: Some(Content::Text(
            "<tool_calls><tool_call>{\"name\": \"get_weather\", \"arguments\": {\"location\": \"Boston\"}}</tool_call></tool_calls>".into(),
        )),
        tool_calls: None,
    };
    let completion = create_test_completion(message, "stop");

    let parsed_completion = completion.parse().unwrap();

    let choice = &parsed_completion.choices[0];
    // finish_reason is modified to "tool_calls" because the content is consumed by tool_calls_response
    assert_eq!(choice.finish_reason, "tool_calls".into());
    if let Message::Assistant { content, tool_calls } = &choice.message {
        // content is taken when tool calls are parsed
        assert!(content.is_none());
        let tool_calls = tool_calls.as_ref().unwrap();
        assert_eq!(tool_calls.len(), 1);
        if let straico_client::endpoints::completion::completion_response::ToolCall::Function { function, .. } = &tool_calls[0] {
            assert_eq!(function.name, "get_weather");
            let expected_args: serde_json::Value = serde_json::json!({"location": "Boston"});
            assert_eq!(function.arguments, expected_args);
        } else {
            panic!("Expected a function tool call");
        }
    } else {
        panic!("Expected an assistant message");
    }
}

#[test]
fn test_completion_parse_finish_reason_end_turn() {
    let message = Message::Assistant {
        content: Some(Content::Text("Hello".into())),
        tool_calls: None,
    };
    let completion = create_test_completion(message, "end_turn");

    let parsed_completion = completion.parse().unwrap();

    let choice = &parsed_completion.choices[0];
    assert_eq!(choice.finish_reason, "stop".into());
}

#[test]
fn test_completion_parse_finish_reason_no_content() {
    let message = Message::Assistant {
        content: None,
        tool_calls: None,
    };
    let completion = create_test_completion(message, "stop");

    let parsed_completion = completion.parse().unwrap();

    let choice = &parsed_completion.choices[0];
    assert_eq!(choice.finish_reason, "tool_calls".into());
}
