use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserData {
    first_name: String,
    last_name: String,
    coins: f32,
    plan: String,
}

impl UserData {
    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }
    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }
    pub fn get_coins(&self) -> f32 {
        self.coins
    }
    pub fn get_plan(&self) -> &str {
        &self.plan
    }
}
