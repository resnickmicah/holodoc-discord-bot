use super::*;

/// Find a restaurant in Ann Arbor for lunch or dinner.
#[poise::command(slash_command)]
pub async fn feedme(ctx: Context<'_>, health_level: Option<String>) -> Result<(), Error> {
    let food_options: HashSet<String> = match health_level {
        Some(health_level) => match health_level.as_str() {
            "healthy" => HashSet::from_iter(FEEDME.healthy.clone()),
            "unhealthy" => HashSet::from_iter(FEEDME.unhealthy.clone()),
            "junk" => HashSet::from_iter(FEEDME.junk.clone()),
            "local" => HashSet::from_iter(FEEDME.local.clone()),
            _ => HashSet::from_iter(vec![
                "Invalid argument. Please choose healthy, unhealthy, junk, or local.".to_string(),
            ]),
        },
        None => HashSet::from_iter(
            [
                FEEDME.healthy.clone(),
                FEEDME.unhealthy.clone(),
                FEEDME.junk.clone(),
                FEEDME.local.clone(),
            ]
            .concat(),
        ),
    };

    let resp = Vec::from_iter(food_options)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    ctx.say(resp).await?;
    Ok(())
}
