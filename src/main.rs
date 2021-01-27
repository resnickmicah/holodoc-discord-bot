use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    Args,
    macros::{
        command,
        group
    }
};
use rand::seq::SliceRandom;

use std::env;
const HEALTHY: &[&str] = &["Japanese", "Mediterranean", "Soup and Salad", "Noodles & Co."];
const LESS_HEALTHY: &[&str] = &["Mexican", "Thai", "Chinese", "Barbecue", "Korean", "Deli"];
const FAST_FOOD: &[&str] = &["Wendy's", "Taco Bell", "Culver's", ];

#[group]
#[commands(feedme)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!!")) // set the bot's prefix to "!!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_HD_TOKEN").expect("token");
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

#[command]
async fn feedme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let food_options: Vec<&str> = if args.len() > 0 {
        let health_level = args.single::<String>()?;
        match health_level.as_str() {
        "healthy" => HEALTHY.to_vec(),
        "unhealthy" => LESS_HEALTHY.to_vec(),
        "junk" => FAST_FOOD.to_vec(),
        _ => vec!["Invalid argument. Please choose healthy, unhealthy, or junk"],
        }

    } else {
        HEALTHY.iter().chain(LESS_HEALTHY).chain(FAST_FOOD).map(|sa| *sa).collect()
    };
    let resp = food_options.choose(&mut rand::thread_rng()).unwrap().to_string();
    if let Err(why) = msg.reply(ctx, resp).await {
        println!("An error occurred while trying to process !feedme: {:?}", why);
    }

    Ok(())
}
