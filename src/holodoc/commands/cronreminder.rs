use super::*;

/**
 * Cronreminder
 */
#[command]
#[description = "Set up a reminder on a regular cadence according to crontab syntax.\n\
    cron [cadence] [reminder message text] [Discord channel]\n\
    Cron examples and analyzer can be found here: https://crontab.guru/"]
pub async fn cronreminder(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("The message to cronreminder command: {:?}", args.message());

    let args = Args::new(args.message(), &[Delimiter::Single('|')]);
    let parsed_args: Vec<&str> = args.raw().collect();
    let resp = if parsed_args.len() == 3 {
        let (cron, reminder, channel) = (parsed_args[0], parsed_args[1], parsed_args[2]);
        format!(
            "cron: {:}\nreminder: {:}\nchannel: {:}\n",
            cron, reminder, channel
        )
    } else {
        format!("Invalid arguments: {:}", args.message())
    };

    if let Err(why) = msg.reply(ctx, resp).await {
        println!(
            "An error occurred while trying to process {:}cronreminder: {:?}",
            BOT_PREFIX, why,
        );
    }

    Ok(())
}
