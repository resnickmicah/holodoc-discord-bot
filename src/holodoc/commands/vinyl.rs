use super::collection::{get_all_releases, print_releases, Album, CollectionReleasesResponse};

use super::*;

/// Picks four random albums from Discogs API
#[poise::command(slash_command)]
pub async fn vinyl(ctx: Context<'_>) -> Result<(), Error> {
    let output = match get_all_releases().await {
        Ok(res) => print_releases(res),
        Err(err) => err.to_string(),
    };
    ctx.say(output).await?;
    Ok(())
}
