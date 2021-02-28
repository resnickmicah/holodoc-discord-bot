use super::*;

#[derive(Serialize, Deserialize)]
pub struct Eateries {
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Foods {
    pub healthy: Vec<String>,
    pub less_healthy: Vec<String>,
    pub fast_food: Vec<String>,
}