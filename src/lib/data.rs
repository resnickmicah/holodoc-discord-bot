use super::*;

lazy_static! {
    pub static ref FEEDME: FeedMe = {
        let feedme = include_str!("../../db/feedme.json");
        let feedme: FeedMe = json::from_str(feedme).expect("Couldn't parse json.");
        feedme
    };
}
