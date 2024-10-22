use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub first_name: String,
    pub last_name: String,
    pub coins: f32,
    pub plan: String,
}
