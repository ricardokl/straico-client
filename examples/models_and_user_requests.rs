use anyhow::Result;
use crate::StraicoClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    let client = StraicoClient::new();

    // Set the API key (replace "your_api_key_here" with your actual API key)
    let client = client.bearer_auth("your_api_key_here");

    // Fetch available models
    let models_response = client.models().send().await?;
    println!("Available Models: {:?}", models_response);

    // Fetch user information
    let user_response = client.user().send().await?;
    println!("User Information: {:?}", user_response);

    Ok(())
}
