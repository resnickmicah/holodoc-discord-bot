/* ========================================
 * Imports
======================================== */
use std::collections::HashSet;
use std::iter::FromIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use serde_json as json;
use serenity::async_trait;
use serenity::client::EventHandler;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

mod holodoc;
use holodoc::{
    commands::{compiling::*, cronreminder::*, feedme::*, pick::*, roll::*, wutplay::*},
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

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> ShuttleSerenity {

    let token = secrets.get("DISCORD_TOKEN").context("missing DISCORD_TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                feedme(),
                cronreminder(),
                wutplay(),
                compiling(),
                pick(),
                roll(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(BOT_PREFIX.into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    Ok(client.into())
}
