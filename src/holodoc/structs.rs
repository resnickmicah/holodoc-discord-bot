use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedMe {
    pub healthy: Vec<String>,
    pub unhealthy: Vec<String>,
    pub junk: Vec<String>,
    pub local: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WutPlay {
    pub vr: Vec<String>,
    pub jrpg: Vec<String>,
    pub arpg: Vec<String>,
    pub coop: Vec<String>,
    pub shooter: Vec<String>,
    pub ragequit: Vec<String>,
    pub tactics: Vec<String>,
    pub chill: Vec<String>,
    pub space: Vec<String>,
    pub steamdeck: Vec<String>,
}
