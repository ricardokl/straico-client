pub mod chat;
pub mod client;
pub mod endpoints;
pub mod error;

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
    /// Converts an endpoint enum variant into its corresponding relative path string
    ///
    /// # Returns
    /// A string slice containing the relative path for the endpoint
    fn as_ref(&self) -> &str {
        match self {
            #[cfg(feature = "user")]
            GetEndpoint::User => "/v0/user",
            #[cfg(feature = "model")]
            GetEndpoint::Models => "/v1/models",
            #[cfg(feature = "rag")]
            GetEndpoint::RagList => "/v0/rag/user",
            #[cfg(feature = "rag")]
            GetEndpoint::RagById => "/v0/rag",
            #[cfg(feature = "agent")]
            GetEndpoint::AgentDetails => "/v0/agent",
            #[cfg(feature = "agent")]
            GetEndpoint::AgentList => "/v0/agent",
        }
    }
}

impl AsRef<str> for PostEndpoint {
    /// Converts a PostEndpoint enum variant into its corresponding relative path string
    ///
    /// # Returns
    /// A string slice containing the relative path for the endpoint
    fn as_ref(&self) -> &str {
        match self {
            PostEndpoint::Completion => "/v1/prompt/completion",
            #[cfg(feature = "image")]
            PostEndpoint::Image => "/v0/image/generation",
            #[cfg(feature = "file")]
            PostEndpoint::File => "/v0/file/upload",
            #[cfg(feature = "rag")]
            PostEndpoint::RagCreate => "/v0/rag",
            #[cfg(feature = "rag")]
            PostEndpoint::RagCompletion => "/v0/rag",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentCreate => "/v0/agent",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentAddRag => "/v0/agent",
            #[cfg(feature = "agent")]
            PostEndpoint::AgentCompletion => "/v0/agent",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "user")]
    fn test_get_endpoint_user_path() {
        assert_eq!(GetEndpoint::User.as_ref(), "/v0/user");
    }

    #[test]
    #[cfg(feature = "model")]
    fn test_get_endpoint_models_path() {
        assert_eq!(GetEndpoint::Models.as_ref(), "/v1/models");
    }

    #[test]
    #[cfg(feature = "rag")]
    fn test_get_endpoint_raglist_path() {
        assert_eq!(GetEndpoint::RagList.as_ref(), "/v0/rag/user");
    }

    #[test]
    #[cfg(feature = "rag")]
    fn test_get_endpoint_ragbyid_path() {
        assert_eq!(GetEndpoint::RagById.as_ref(), "/v0/rag");
    }

    #[test]
    #[cfg(feature = "agent")]
    fn test_get_endpoint_agentdetails_path() {
        assert_eq!(GetEndpoint::AgentDetails.as_ref(), "/v0/agent");
    }

    #[test]
    #[cfg(feature = "agent")]
    fn test_get_endpoint_agentlist_path() {
        assert_eq!(GetEndpoint::AgentList.as_ref(), "/v0/agent");
    }

    #[test]
    fn test_post_endpoint_completion_path() {
        assert_eq!(PostEndpoint::Completion.as_ref(), "/v1/prompt/completion");
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_post_endpoint_image_path() {
        assert_eq!(PostEndpoint::Image.as_ref(), "/v0/image/generation");
    }

    #[test]
    #[cfg(feature = "file")]
    fn test_post_endpoint_file_path() {
        assert_eq!(PostEndpoint::File.as_ref(), "/v0/file/upload");
    }

    #[test]
    #[cfg(feature = "rag")]
    fn test_post_endpoint_ragcreate_path() {
        assert_eq!(PostEndpoint::RagCreate.as_ref(), "/v0/rag");
    }

    #[test]
    #[cfg(feature = "rag")]
    fn test_post_endpoint_ragcompletion_path() {
        assert_eq!(PostEndpoint::RagCompletion.as_ref(), "/v0/rag");
    }

    #[test]
    #[cfg(feature = "agent")]
    fn test_post_endpoint_agentcreate_path() {
        assert_eq!(PostEndpoint::AgentCreate.as_ref(), "/v0/agent");
    }

    #[test]
    #[cfg(feature = "agent")]
    fn test_post_endpoint_agentaddrag_path() {
        assert_eq!(PostEndpoint::AgentAddRag.as_ref(), "/v0/agent");
    }

    #[test]
    #[cfg(feature = "agent")]
    fn test_post_endpoint_agentcompletion_path() {
        assert_eq!(PostEndpoint::AgentCompletion.as_ref(), "/v0/agent");
    }
}
