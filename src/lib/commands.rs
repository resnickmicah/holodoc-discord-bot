use super::*;

/**
 * Feedme
 */
#[command]
pub async fn feedme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let food_options: Vec<String> = if args.len() > 0 {
        let health_level = args.single::<String>()?;
        match health_level.as_str() {
            "healthy" => FEEDME.healthy.clone(),
            "unhealthy" => FEEDME.unhealthy.clone(),
            "junk" => FEEDME.junk.clone(),
            "local" => FEEDME.local.clone(),
            _ => vec!["Invalid argument. Please choose healthy, unhealthy, junk, or local".to_string()],
        }
    } else {
        [FEEDME.healthy.clone(), FEEDME.unhealthy.clone(), FEEDME.junk.clone(), FEEDME.local.clone()].concat()
    };

    let resp = food_options
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    if let Err(e) = msg.reply(ctx, resp).await {
        println!("An error occurred while trying to process feedme: {:?}", e);
    }

    Ok(())
}

/**
 * Cronreminder
 */
#[command]
async fn cronreminder(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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
 