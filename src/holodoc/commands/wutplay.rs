use super::*;
/// Pick a game to play. With no args, selects a random game among all tags.
#[poise::command(
    slash_command,
    aliases("wut2play", "watplay", "idkwut2play", "playwat", "whattoplay?")
)]
pub async fn wutplay(ctx: Context<'_>, genre: Option<String>) -> Result<(), Error> {
    let game_options: HashSet<String> = match genre {
        Some(genre) => match genre.as_str() {
            "vr" => HashSet::from_iter(WUTPLAY.vr.clone()),
            "jrpg" => HashSet::from_iter(WUTPLAY.jrpg.clone()),
            "arpg" => HashSet::from_iter(WUTPLAY.arpg.clone()),
            "rpg" => HashSet::from_iter([WUTPLAY.arpg.clone(), WUTPLAY.jrpg.clone()].concat()),
            "coop" => HashSet::from_iter(WUTPLAY.coop.clone()),
            "shooter" => HashSet::from_iter(WUTPLAY.shooter.clone()),
            "ragequit" => HashSet::from_iter(WUTPLAY.ragequit.clone()),
            "tactics" => HashSet::from_iter(WUTPLAY.tactics.clone()),
            "chill" => HashSet::from_iter(WUTPLAY.chill.clone()),
            "space" => HashSet::from_iter(WUTPLAY.space.clone()),
            "steamdeck" => HashSet::from_iter(WUTPLAY.steamdeck.clone()),
            _ => HashSet::from_iter(
                vec!["Invalid argument. Please choose vr, rpg, jrpg, arpg, coop, shooter, ragequit, tactics, space, steamdeck, or chill.".to_string()]
            ),
        },
        None => HashSet::from_iter(
            [
                WUTPLAY.vr.clone(),
                WUTPLAY.jrpg.clone(),
                WUTPLAY.arpg.clone(),
                WUTPLAY.coop.clone(),
                WUTPLAY.shooter.clone(),
                WUTPLAY.ragequit.clone(),
                WUTPLAY.tactics.clone(),
                WUTPLAY.chill.clone(),
                WUTPLAY.space.clone(),
            ]
            .concat(),
        )
    };

    let resp = Vec::from_iter(game_options)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    ctx.say(resp).await?;

    Ok(())
}
