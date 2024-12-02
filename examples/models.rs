use anyhow::Result;
#[cfg(feature = "model")]
use straico::client::StraicoClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    #[cfg(feature = "model")]
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    #[cfg(feature = "model")]
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Fetch available models
    #[cfg(feature = "model")]
    let models_response = client.clone().models().bearer_auth(&api_key).send().await?;
    #[cfg(feature = "model")]
    println!("Available Models: {:#?}", models_response);

    Ok(())
}
