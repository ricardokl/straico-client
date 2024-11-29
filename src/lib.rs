pub mod chat;
pub mod client;
pub mod endpoints;

/// Represents endpoints for GET requests to the Straico API
///
/// # Variants
/// * `User` - Endpoint for user information
/// * `Models` - Endpoint for available models
pub enum GetEndpoint {
    User,
    Models,
}

/// Represents endpoints for POST requests to the Straico API
///
/// # Variants
/// * `Image` - Endpoint for image generation
/// * `Completion` - Endpoint for prompt completion
/// * `File` - Endpoint for file uploads
pub enum PostEndpoint {
    Image,
    Completion,
    File,
}

impl AsRef<str> for GetEndpoint {
    /// Converts an endpoint enum variant into its corresponding URL string
    ///
    /// # Returns
    /// A string slice containing the full URL for the endpoint
    fn as_ref(&self) -> &str {
        match self {
            GetEndpoint::User => "https://api.straico.com/v0/user",
            GetEndpoint::Models => "https://api.straico.com/v1/models",
        }
    }
}

impl AsRef<str> for PostEndpoint {
    /// Converts a PostEndpoint enum variant into its corresponding URL string
    ///
    /// # Returns
    /// A string slice containing the full URL for the endpoint
    fn as_ref(&self) -> &str {
        match self {
            PostEndpoint::Image => "https://api.straico.com/v0/image/generation",
            PostEndpoint::Completion => "https://api.straico.com/v1/prompt/completion",
            PostEndpoint::File => "https://api.straico.com/v0/file/upload",
        }
    }
}
