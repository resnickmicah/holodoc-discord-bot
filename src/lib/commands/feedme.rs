use super::*;

/**
 * Feedme
 */
#[command]
#[description = "Find a restaurant in Ann Arbor for lunch or dinner.\n\
    With no args, selects a random restaurant type or specific restaurant.\n\
    Possible args are healthy, unhealthy, junk, or local."]
pub async fn feedme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let food_options: Vec<String> = if args.len() > 0 {
        let health_level = args.single::<String>()?;
        match health_level.as_str() {
            "healthy" => FEEDME.healthy.clone(),
            "unhealthy" => FEEDME.unhealthy.clone(),
            "junk" => FEEDME.junk.clone(),
            "local" => FEEDME.local.clone(),
            _ => vec![
                "Invalid argument. Please choose healthy, unhealthy, junk, or local".to_string(),
            ],
        }
    } else {
        [
            FEEDME.healthy.clone(),
            FEEDME.unhealthy.clone(),
            FEEDME.junk.clone(),
            FEEDME.local.clone(),
        ]
        .concat()
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
