pub mod client;
pub mod common;
pub mod endpoints;

pub const BASE_URL: &str = "https://platform.neural.space";

pub enum GetEndpoint {
    User,
    Models,
    File,
}

pub enum PostEndpoint {
    Image,
    Completion,
}

impl AsRef<str> for GetEndpoint {
    fn as_ref(&self) -> &str {
        match self {
            GetEndpoint::User => "/v0/user",
            GetEndpoint::Models => "/v1/models",
            GetEndpoint::File => "/v0/file/upload",
        }
    }
}

impl AsRef<str> for PostEndpoint {
    fn as_ref(&self) -> &str {
        match self {
            PostEndpoint::Image => "/v0/image/generation",
            PostEndpoint::Completion => "/v1/prompt/completion",
        }
    }
}
