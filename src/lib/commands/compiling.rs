use super::*;

#[command]
#[description = "Shows my favorite comic about compiling"]
pub async fn compiling(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, "https://imgs.xkcd.com/comics/compiling.png").await {
        println!("Uhh... something's pretty fucked if in my photonic matrix if I can't even print a URL: {:}", why);
    }

    Ok(())
}
