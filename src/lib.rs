pub mod client;
pub mod common;
pub mod endpoints;

pub enum GetEndpoint {
    User,
    Models,
}

pub enum PostEndpoint {
    Image,
    Completion,
    File,
}

impl AsRef<str> for GetEndpoint {
    fn as_ref(&self) -> &str {
        match self {
            GetEndpoint::User => "https://api.straico.com/v0/user",
            GetEndpoint::Models => "https://api.straico.com/v1/models",
        }
    }
}

impl AsRef<str> for PostEndpoint {
    fn as_ref(&self) -> &str {
        match self {
            PostEndpoint::Image => "https://api.straico.com/v0/image/generation",
            PostEndpoint::Completion => "https://api.straico.com/v1/prompt/completion",
            PostEndpoint::File => "https://api.straico.com/v0/file/upload",
        }
    }
}
