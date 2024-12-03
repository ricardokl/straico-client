use anyhow::Result;
use straico::client::StraicoClient;
use straico::endpoints::completion::completion_request::CompletionRequest;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Create a completion request
    let completion_request = CompletionRequest {
        model: "completion-model-1".to_string(),
        prompt: "Once upon a time in a land far, far away".to_string(),
        max_tokens: 50,
        temperature: 0.7,
        top_p: 1.0,
        n: 1,
        stop: None,
        echo: false,
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
        logit_bias: None,
        user: None,
    };

    // Send the completion request to the API
    let completion_response = client
        .completion()
        .bearer_auth(&api_key)
        .json(completion_request)
        .send()
        .await?;

    // Print the response
    println!("Completion Response:\n\n{:#?}", completion_response);

    Ok(())
}
