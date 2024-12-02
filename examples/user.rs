use anyhow::Result;
#[cfg(feature = "user")]
use straico::client::StraicoClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    #[cfg(feature = "user")]
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    #[cfg(feature = "user")]
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Fetch user information
    #[cfg(feature = "user")]
    let user_response = client.user().bearer_auth(api_key).send().await?;
    #[cfg(feature = "user")]
    println!("User Information:\n\n{:#?}", user_response);

    Ok(())
}
