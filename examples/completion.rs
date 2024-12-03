use anyhow::Result;
use straico::client::StraicoClient;
use straico::endpoints::completion::completion_request::CompletionRequest;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Create a completion request with a single model
    // The .models() method accepts both strings and arrays
    let completion_request = CompletionRequest::new()
        // Required field
        .models("openai/gpt-3.5-turbo-0125")
        // Required field
        .message("What is the answer to life, the universe, and everything?")
        // Optional field
        .temperature(0.5)
        // Optional field
        .max_tokens(1000)
        // Need to build at the end
        .build();

    // Cloning the client is a cheap operation as StraicoClient implements Clone
    // and only contains lightweight internal references. This allows us to reuse
    // the client for multiple requests without performance impact.
    let completion_response = client
        .clone()
        .completion()
        .bearer_auth(&api_key)
        .json(completion_request)
        .send()
        .await?;

    // Print the response
    println!("Completion Response:\n\n{:#?}", completion_response);

    // Create a completion request with multiple models
    // Notice how .models() accepts an array of strings
    let completion_request = CompletionRequest::new()
        // Required field
        .models(["openai/gpt-3.5-turbo-0125", "anthropic/claude-3.5-sonnet"]) // Array of model strings
        // Required field
        .message("What is the answer to life, the universe, and everything?")
        // Optional field, notice max_tokens was not defined
        .temperature(0.5)
        .build();

    // Send the completion request to the API
    let completion_response = client
        .completion()
        .bearer_auth(&api_key)
        .json(completion_request)
        .send()
        .await?;

    // Print the response
    println!(
        "\n\nMulti-model Completion Response:\n\n{:#?}",
        completion_response
    );

    Ok(())
}
