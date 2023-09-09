use super::*;

#[command]
#[description = "pick a random word from the args"]
async fn pick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let args: Vec<String> = args.iter().map(|a| a.unwrap()).collect();
    let rand_arg = args.choose(&mut rand::thread_rng()).unwrap();
    let response = format!("{}", rand_arg);

    if let Err(why) = msg.reply(ctx, response).await {
        println!(
            "An error occurred while processing {:}pick: {:?}",
            BOT_PREFIX, why
        );
    }
    Ok(())
}
