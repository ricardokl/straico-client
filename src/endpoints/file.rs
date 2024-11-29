use anyhow::Result;
use serde::Deserialize;
use std::{fmt::Display, path::Path};

use crate::client::{ApiKeySet, PayloadSet, StraicoClient, StraicoRequestBuilder};

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub url: String,
}

pub struct FileRequest {
    file: String,
}
pub struct FileRequestBuilder<T> {
    file: T,
}
pub struct FileNotSet;

impl FileRequest {
    pub fn new() -> FileRequestBuilder<FileNotSet> {
        FileRequestBuilder { file: FileNotSet }
    }
}

impl FileRequestBuilder<FileNotSet> {
    pub fn file<U: AsRef<Path>>(self, file: U) -> FileRequestBuilder<U> {
        FileRequestBuilder { file }
    }
}

impl FileRequestBuilder<String> {
    pub fn build(self) -> FileRequest {
        FileRequest { file: self.file }
    }

    pub async fn bearer_auth<T: Display>(
        self,
        key: T,
    ) -> Result<StraicoRequestBuilder<ApiKeySet, PayloadSet, FileData>> {
        let file = self.build().file;
        let client = StraicoClient::new();
        client.file().bearer_auth(key).multipart(file).await
    }
}
