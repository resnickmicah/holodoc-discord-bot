/* ========================================
 * Imports
======================================== */
mod lib;
use dotenv;
use lib::{
    commands::*,
    structs::{Eateries, Foods},
};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json as json;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, Delimiter, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{async_trait, prelude::TypeMapKey};
use serenity::{
    client::{Client, Context, EventHandler},
    prelude::RwLock,
};

use std::{collections::HashMap, fs, sync::Arc};
use std::{env, path::Path};

/* ========================================
 * Consts
======================================== */
const BOT_PREFIX: &str = "!!";

#[macro_use]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

/* ========================================
 * State
======================================== */
struct FoodsState;
impl TypeMapKey for FoodsState {
    // We need to implement typemapkey to avoid the bot deadlocking.
    type Value = Arc<RwLock<HashMap<String, Foods>>>;
}

struct EateryState;
impl TypeMapKey for EateryState {
    // We need to implement typemapkey to avoid the bot deadlocking.
    type Value = Arc<RwLock<HashMap<String, Eateries>>>;
}

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

    // Initialize data
    {
        let foods = &fs::read_to_string(Path::new("./db/foods.json")).expect("Couldn't read file.");
        let foods: Foods = json::from_str(foods).expect("Couldn't parse json.");

        let eatery =
            &fs::read_to_string(Path::new("./db/eatery.json")).expect("Couldn't read file.");
        let eatery: Eateries = json::from_str(eatery).expect("Couldn't parse json.");

        let mut data = client.data.write().await;

        data.insert::<FoodsState>(Arc::new(RwLock::new(
            hashmap!["foods".to_string() => foods],
        )));
        data.insert::<EateryState>(Arc::new(RwLock::new(
            hashmap!["eatery".to_string() => eatery],
        )));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
