use straico_client::client::StraicoClient;
use straico_client::endpoints::completion::completion_request::CompletionRequest;
use straico_client::endpoints::ApiResponseData;
use mockito::mock;

#[tokio::test]
async fn test_client_completion_request() {
    let _m = mock("POST", "/v1/prompt/completion")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "success": true,
            "data": {
                "completions": {
                    "0": {
                        "completion": {
                            "choices": [
                                {
                                    "finish_reason": "stop",
                                    "index": 0,
                                    "message": {
                                        "content": "Hello, world!",
                                        "role": "assistant"
                                    }
                                }
                            ],
                            "created": 1677652288,
                            "id": "chatcmpl-123",
                            "model": "gpt-3.5-turbo-0125",
                            "object": "chat.completion",
                            "usage": {
                                "completion_tokens": 9,
                                "prompt_tokens": 10,
                                "total_tokens": 19
                            }
                        },
                        "price": {
                            "input": 0.0,
                            "output": 0.0,
                            "total": 0.0
                        },
                        "words": {
                            "input": 2,
                            "output": 2,
                            "total": 4
                        }
                    }
                },
                "overall_price": {
                    "input": 0.0,
                    "output": 0.0,
                    "total": 0.0
                },
                "overall_words": {
                    "input": 2,
                    "output": 2,
                    "total": 4
                }
            }
        }"#)
        .create();

    let client = StraicoClient::new_with_host(&mockito::server_url());
    let request = CompletionRequest::new()
        .models(["openai/gpt-3.5-turbo"])
        .message("Hello")
        .build();

    let response = client
        .completion()
        .bearer_auth("test-api-key")
        .json(request)
        .send()
        .await
        .unwrap();

    let completion = response.get_completion().unwrap();
    assert_eq!(completion.id, "chatcmpl-123".into());
    assert_eq!(completion.choices[0].message.content.as_ref().unwrap().to_string(), "Hello, world!");
}
