use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct FileData {
    pub url: String,
}

#[allow(dead_code)]
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
}
