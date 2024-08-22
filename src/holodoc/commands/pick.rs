use errors::HolodocErrors;

use super::*;

/// pick a random word from the args
#[poise::command(slash_command)]
pub async fn pick(ctx: Context<'_>, options: String) -> Result<(), Error> {
    let user_nick = ctx.author();
    let options: Vec<&str> = options.split(' ').collect();
    let rand_arg = options
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| HolodocErrors::RNGFailure)?;
    let response = format!(
        "{} asked me to pick between: {:?}.\nI pick: {}",
        user_nick, options, rand_arg
    );
    ctx.say(response).await?;
    Ok(())
}
