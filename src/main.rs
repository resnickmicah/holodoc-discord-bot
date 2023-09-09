/* ========================================
 * Imports
======================================== */
use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;

use dotenv;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json as json;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, Delimiter, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;

mod holodoc;
use holodoc::{
    commands::{compiling::*, cronreminder::*, feedme::*, help::*, pick::*, roll::*, wutplay::*},
    data::*,
    structs::{FeedMe, WutPlay},
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
#[commands(feedme, cronreminder, wutplay, compiling, pick, roll)]
pub struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(BOT_PREFIX)) // set the bot's prefix to "!!"
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

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
