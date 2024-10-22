use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub url: String,
}
