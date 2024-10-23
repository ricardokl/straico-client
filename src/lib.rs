mod common;
mod client;
pub use common::ApiResponse;
pub use client::Client;
pub mod endpoints;

pub const BASE_URL: &str = "https://api.straico.com";

pub enum V0 {
    User,
    Image,
    File,
}

pub enum V1 {
    Models,
    Completion,
}

impl AsRef<str> for V0 {
    fn as_ref(&self) -> &str {
        match self {
            V0::User => "/v0/user",
            V0::Image => "/v0/image/generation",
            V0::File => "/v0/file/upload",
        }
    }
}

impl AsRef<str> for V1 {
    fn as_ref(&self) -> &str {
        match self {
            V1::Models => "/v1/models",
            V1::Completion => "/v1/prompt/completion",
        }
    }
}
