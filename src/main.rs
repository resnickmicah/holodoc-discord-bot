/* ========================================
 * Imports
======================================== */
use std::env;

use dotenv;

use rand::seq::SliceRandom;

use serde::{Deserialize, Serialize};
use serde_json as json;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, Delimiter, StandardFramework,
};
use serenity::model::channel::Message;

mod lib;
use lib::{
    commands::{cronreminder::*, feedme::*},
    data::*,
    structs::FeedMe,
};

#[macro_use]
extern crate lazy_static;

/* ========================================
 * Consts
======================================== */
const BOT_PREFIX: &str = "!!";

/* ========================================
 * Program
======================================== */
#[group]
#[commands(feedme, cronreminder)]
pub struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(BOT_PREFIX)) // set the bot's prefix to "!!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
