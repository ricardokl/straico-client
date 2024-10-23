use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub coins: f32,
    pub plan: String,
}
