use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::{fmt::Display, io::Error, path::Path};

use crate::client::{ApiKeySet, IntoStraicoClient, PayloadSet, StraicoRequestBuilder};

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
    ) -> Result<StraicoRequestBuilder<ApiKeySet, PayloadSet, FileData>, Error> {
        let file = self.build().file;
        ReqwestClient::new()
            .to_straico()
            .file()
            .bearer_auth(key)
            .form(file)
            .await
    }
}
