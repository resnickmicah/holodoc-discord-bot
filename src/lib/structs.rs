use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedMe {
    pub healthy: Vec<String>,
    pub unhealthy: Vec<String>,
    pub junk: Vec<String>,
    pub local: Vec<String>,
}
