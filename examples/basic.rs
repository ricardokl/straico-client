use straico::{Client, endpoints::{
    image::ImageRequest,
    completion::CompletionRequest,
}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");
    let client = Client::new(api_key);

    // Create an image
    let image_request = ImageRequest {
        model: "dall-e-3".to_string(),
        description: "A cute cat playing with a ball of yarn".to_string(),
        size: "1024x1024".to_string(),
        variations: 1,
    };
    let image_response = client.create_image(&image_request).await?;
    println!("Generated {} images", image_response.images.len());

    // Create a completion
    let completion_request = CompletionRequest {
        models: vec!["gpt-4".to_string()],
        file_urls: vec![],
        youtube_urls: vec![],
        message: "What is the meaning of life?".to_string(),
        display_transcripts: false,
        temperature: 0.7,
        max_tokens: 1000,
    };
    let completion_response = client.create_completion(&completion_request).await?;
    println!("Got {} completions", completion_response.completions.len());

    Ok(())
}
