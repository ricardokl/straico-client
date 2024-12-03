use anyhow::Result;
#[cfg(feature = "file")]
use straico::client::StraicoClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the StraicoClient
    #[cfg(feature = "file")]
    let client = StraicoClient::new();

    // Fetch API key from environment variable
    #[cfg(feature = "file")]
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");

    // Create a file request using the builder pattern
    // #[cfg(feature = "file")]
    // let file_request = FileRequest::new().file("path/to/your/file.txt");

    // Send the file request to the API
    #[cfg(feature = "file")]
    let file_response = client
        .file()
        .bearer_auth(api_key)
        .multipart("path/to/file.txt")
        .await?
        .send()
        .await?;

    // Print the response
    #[cfg(feature = "file")]
    println!("File Upload Response:\n\n{:#?}", file_response);

    Ok(())
}
