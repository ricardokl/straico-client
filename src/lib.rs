mod client;
pub mod common;

pub use client::Client;
pub use common::ApiResponse;
pub mod endpoints;

pub const BASE_URL: &str = "https://api.straico.com/";

pub enum Endpoint {
    User,
    Image,
    File,
    Models,
    Completion,
}

impl AsRef<str> for Endpoint {
    fn as_ref(&self) -> &str {
        match self {
            Endpoint::User => "/v0/user",
            Endpoint::Image => "/v0/image/generation",
            Endpoint::File => "/v0/file/upload",
            Endpoint::Models => "/v1/models",
            Endpoint::Completion => "/v1/prompt/completion",
        }
    }
}
