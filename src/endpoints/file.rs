use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub url: String,
}

pub struct FileRequest;
