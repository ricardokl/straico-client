use anyhow::Result;
#[cfg(feature = "image")]
use straico::client::StraicoClient;
#[cfg(feature = "image")]
use straico::endpoints::image::{ImageRequest, ImageRequestBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    #[cfg(feature = "image")]
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    #[cfg(feature = "image")]
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Create an image request using the builder pattern
    #[cfg(feature = "image")]
    let image_request = ImageRequest::new()
        .model("image-model-1")
        .description("A futuristic cityscape with flying cars")
        .size("1024x1024")
        .variations(3)
        .build();

    // Send the image request to the API
    #[cfg(feature = "image")]
    let image_response = client
        .image()
        .bearer_auth(&api_key)
        .json(image_request)
        .send()
        .await?;

    // Print the response
    #[cfg(feature = "image")]
    println!("Image Generation Response:\n\n{:#?}", image_response);

    Ok(())
}
