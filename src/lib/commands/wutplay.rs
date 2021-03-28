use super::*;

/**
 * Feedme
 */
#[command]
#[description = "Pick a game to play\n\
    With no args, selects a random game among all tags.\n\
    Possible genres are vr, jrpg, arpg, coop, shooter, ragequit, tactics, and chill."]
pub async fn wutplay(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let food_options: HashSet<String> = if args.len() > 0 {
        let health_level = args.single::<String>()?;
        match health_level.as_str() {
            "vr" => HashSet::from_iter(WUTPLAY.vr.clone()),
            "jrpg" => HashSet::from_iter(WUTPLAY.jrpg.clone()),
            "arpg" => HashSet::from_iter(WUTPLAY.arpg.clone()),
            "rpg" => HashSet::from_iter([WUTPLAY.arpg.clone(), WUTPLAY.jrpg.clone()].concat()),
            "coop" => HashSet::from_iter(WUTPLAY.coop.clone()),
            "shooter" => HashSet::from_iter(WUTPLAY.shooter.clone()),
            "ragequit" => HashSet::from_iter(WUTPLAY.ragequit.clone()),
            "tactics" => HashSet::from_iter(WUTPLAY.tactics.clone()),
            "chill" => HashSet::from_iter(WUTPLAY.chill.clone()),
            _ => HashSet::from_iter(
                vec!["Invalid argument. Please choose vr, jrpg, arpg, coop, shooter, ragequit, tactics, or chill.".to_string()]
            ),
        }
    } else {
        HashSet::from_iter([
            WUTPLAY.vr.clone(),
            WUTPLAY.jrpg.clone(),
            WUTPLAY.arpg.clone(),
            WUTPLAY.coop.clone(),
            WUTPLAY.shooter.clone(),
            WUTPLAY.ragequit.clone(),
            WUTPLAY.tactics.clone(),
            WUTPLAY.chill.clone(),
        ]
        .concat())
    };

    let resp = Vec::from_iter(food_options)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    if let Err(e) = msg.reply(ctx, resp).await {
        println!("An error occurred while trying to process feedme: {:?}", e);
    }

    Ok(())
}
