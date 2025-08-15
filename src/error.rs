use thiserror::Error;

/// A custom error type for the Straico client library.
///
/// This enum represents all possible errors that can occur when using the
/// Straico API client. It is designed to provide detailed, specific error
/// information to the user.
#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred during an I/O operation.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred while making a request with the `reqwest` client.
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// An error occurred during JSON serialization or deserialization.
    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),
}
