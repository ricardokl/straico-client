use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct UserData {
    first_name: String,
    last_name: String,
    coins: f32,
    plan: String,
}
