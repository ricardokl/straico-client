pub mod chat;
pub mod client;
pub mod endpoints;

/// Represents endpoints for GET requests to the Straico API
///
/// # Variants
/// * `User` - Endpoint for user information
/// * `Models` - Endpoint for available models
#[allow(dead_code)]
enum GetEndpoint {
    #[cfg(feature = "user")]
    //#[allow(dead_code)]
    User,
    #[cfg(feature = "model")]
    //#[allow(dead_code)]
    Models,
    #[cfg(feature = "rag")]
    //#[allow(dead_code)]
    RagList,
    #[cfg(feature = "rag")]
    //#[allow(dead_code)]
    RagById,
    #[cfg(feature = "agent")]
    //#[allow(dead_code)]
    AgentDetails,
    #[cfg(feature = "agent")]
    //#[allow(dead_code)]
    AgentList,
}

/// Represents endpoints for POST requests to the Straico API
///
/// # Variants
/// * `Image` - Endpoint for image generation
/// * `Completion` - Endpoint for prompt completion
/// * `File` - Endpoint for file uploads
enum PostEndpoint {
    Completion,
    #[cfg(feature = "image")]
    //#[allow(dead_code)]
    Image,
    #[cfg(feature = "file")]
    //#[allow(dead_code)]
    File,
    #[cfg(feature = "rag")]
    //#[allow(dead_code)]
    RagCreate,
    #[cfg(feature = "rag")]
    //#[allow(dead_code)]
    RagCompletion,
    #[cfg(feature = "agent")]
    //#[allow(dead_code)]
    AgentCompletion,
    #[cfg(feature = "agent")]
    //#[allow(dead_code)]
    AgentCreate,
    #[cfg(feature = "agent")]
    //#[allow(dead_code)]
    AgentAddRag,
}

impl AsRef<str> for GetEndpoint {
    /// Converts an endpoint enum variant into its corresponding URL string
    ///
    /// # Returns
    /// A string slice containing the full URL for the endpoint
    fn as_ref(&self) -> &str {
        match self {
            #[cfg(feature = "user")]
            GetEndpoint::User => "https://api.straico.com/v0/user",
            #[cfg(feature = "model")]
            GetEndpoint::Models => "https://api.straico.com/v1/models",
            #[cfg(feature = "rag")]
            GetEndpoint::RagList => "https://api.straico.com/v0/rag/user",
            #[cfg(feature = "rag")]
            GetEndpoint::RagById => "https://api.straico.com/v0/rag",
            #[cfg(feature = "agent")]
            GetEndpoint::AgentDetails => "https://api.straico.com/v0/agent",
            #[cfg(feature = "agent")]
            GetEndpoint::AgentList => "https://api.straico.com/v0/agent",
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
            PostEndpoint::Completion => "https://api.straico.com/v1/prompt/completion",
            #[cfg(feature = "image")]
            PostEndpoint::Image => "https://api.straico.com/v0/image/generation",
            #[cfg(feature = "file")]
            PostEndpoint::File => "https://api.straico.com/v0/file/upload",
            #[cfg(feature = "rag")]
            PostEndpoint::RagCreate => "https://api.straico.com/v0/rag",
            #[cfg(feature = "rag")]
            PostEndpoint::RagCompletion => "https://api.straico.com/v0/rag",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentCreate => "https://api.straico.com/v0/agent",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentAddRag => "https://api.straico.com/v0/agent",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentCompletion => "https://api.straico.com/v0/agent",
        }
    }
}
