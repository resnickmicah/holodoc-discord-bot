use super::*;

/// pick a random word from the args
#[poise::command(slash_command)]
pub async fn pick(ctx: Context<'_>, options: String) -> Result<(), Error> {
    let options: Vec<&str> = options.split(' ').collect();
    let rand_arg = options.choose(&mut rand::thread_rng()).unwrap();
    let response = format!("{}", rand_arg);
    ctx.say(response).await?;
    Ok(())
}
