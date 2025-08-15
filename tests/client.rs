use straico::client::StraicoClient;
use straico::endpoints::completion::completion_request::CompletionRequest;
use mockito::Server;

#[tokio::test]
async fn test_completion() {
    let mut server = Server::new_async().await;
    let base_url = server.url();

    let mock = server
        .mock("POST", "/v1/prompt/completion")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "success": true,
            "data": {
                "completions": {
                    "test-model": {
                        "completion": {
                            "id": "test-id",
                            "choices": [
                                {
                                    "index": 0,
                                    "message": {
                                        "role": "assistant",
                                        "content": "Hello, world!"
                                    },
                                    "finish_reason": "stop"
                                }
                            ],
                            "created": 123,
                            "model": "test-model",
                            "object": "chat.completion",
                            "usage": {
                                "prompt_tokens": 1,
                                "completion_tokens": 2,
                                "total_tokens": 3
                            }
                        },
                        "price": { "input": 0.0, "output": 0.0, "total": 0.0 },
                        "words": { "input": 1, "output": 2, "total": 3 }
                    }
                },
                "overall_price": { "input": 0.0, "output": 0.0, "total": 0.0 },
                "overall_words": { "input": 1, "output": 2, "total": 3 }
            }
        }"#,
        )
        .create_async()
        .await;

    let client = StraicoClient::with_base_url(base_url);

    let request = CompletionRequest::new()
        .models("test-model")
        .message("test-prompt")
        .build();

    let response = client
        .completion()
        .bearer_auth("test-api-key")
        .json(request)
        .send()
        .await
        .unwrap();

    mock.assert_async().await;

    let completion = response.get_completion().unwrap();
    assert_eq!(completion.model.as_ref(), "test-model");
    let choice = &completion.choices[0];
    match &choice.message {
        straico::endpoints::completion::completion_response::Message::Assistant {
            content, ..
        } => {
            assert_eq!(content.as_ref().unwrap().to_string().trim(), "Hello, world!");
        }
        _ => panic!("Unexpected message type"),
    }
    assert_eq!(completion.usage.total_tokens, 3);
}

#[tokio::test]
async fn test_models() {
    let mut server = Server::new_async().await;
    let base_url = server.url();

    let mock = server
        .mock("GET", "/v1/models")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "success": true,
            "data": {
                "chat": [
                    {
                        "name": "Test Chat Model",
                        "model": "test/chat-model",
                        "word_limit": 1000,
                        "max_output": 2000,
                        "pricing": {
                            "coins": 1.0,
                            "words": 100
                        },
                        "metadata": {}
                    }
                ],
                "image": []
            }
        }"#,
        )
        .create_async()
        .await;

    let client = StraicoClient::with_base_url(base_url);

    let response = client
        .models()
        .bearer_auth("test-api-key")
        .send()
        .await
        .unwrap();

    mock.assert_async().await;

    assert!(response.success);
    if let straico::endpoints::ApiResponseVariant::Data {
        data: straico::endpoints::ResponseType::Model(model_data),
    } = response.response
    {
        let chat_models = model_data.chat();
        assert_eq!(chat_models.len(), 1);
        assert_eq!(chat_models[0].name(), "Test Chat Model");
        assert_eq!(chat_models[0].model(), "test/chat-model");
    } else {
        panic!("Unexpected response type");
    }
}
