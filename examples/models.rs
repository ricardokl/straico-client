use anyhow::Result;
use straico::client::StraicoClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Fetch available models
    let models_response = client.clone().models().bearer_auth(&api_key).send().await?;
    println!("Available Models: {:#?}", models_response);

    Ok(())
}
