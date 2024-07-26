use super::*;
use super::structs::{FeedMe, WutPlay};

lazy_static! {
    pub static ref FEEDME: FeedMe = {
        let feedme = include_str!("../../db/feedme.json");
        let feedme: FeedMe = json::from_str(feedme).expect("Couldn't parse json.");
        feedme
    };
}

lazy_static! {
    pub static ref WUTPLAY: WutPlay = {
        let wutplay = include_str!("../../db/wutplay.json");
        let wutplay: WutPlay = json::from_str(wutplay).expect("Couldn't parse json.");
        wutplay
    };
}
