mod common;
pub use common::ApiResponse;
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
            V0::User => "/user",
            V0::Image => "/image/generation",
            V0::File => "/file/upload",
        }
    }
}

impl AsRef<str> for V1 {
    fn as_ref(&self) -> &str {
        match self {
            V1::Models => "/models",
            V1::Completion => "/prompt/completion",
        }
    }
}
