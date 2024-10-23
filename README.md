# Straico API Client

A Rust client for the Straico API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
straico = "0.1.0"
```

## Usage

```rust
use straico::{Client, endpoints::{
    image::ImageRequest,
    completion::CompletionRequest,
}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("STRAICO_API_KEY").expect("STRAICO_API_KEY must be set");
    let client = Client::new(api_key);

    // Generate an image
    let image_request = ImageRequest {
        model: "dall-e-3".to_string(),
        description: "A cute cat playing with a ball of yarn".to_string(),
        size: "1024x1024".to_string(),
        variations: 1,
    };
    let image_response = client.create_image(&image_request).await?;
    println!("Generated {} images", image_response.images.len());

    Ok(())
}
```

## Features

- Image generation
- Text completion
- File upload support
- Type-safe request and response handling
- Async/await support

## License

This project is licensed under the MIT License.
