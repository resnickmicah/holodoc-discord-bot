use super::*;

/// Shows my favorite comic about compiling
#[poise::command(slash_command)]
pub async fn compiling(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://imgs.xkcd.com/comics/compiling.png")
        .await?;
    Ok(())
}
