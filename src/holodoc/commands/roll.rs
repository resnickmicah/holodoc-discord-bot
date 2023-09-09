use super::*;

#[command]
#[aliases("r")]
#[description = "roll [num dice] [num sides]"]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_dice = args.single::<u16>()?;
    let num_sides = args.single::<u16>()?;
    let mut rolls: Vec<u16> = vec![];
    let mut roll_total: u16 = 0;
    for _ in 1..=num_dice {
        let mut rng = thread_rng();
        let roll_result: u16 = rng.gen_range(1..=num_sides);
        rolls.push(roll_result);
        roll_total += roll_result;
    }
    let response = format!("Roll results: {:?}, total: {}", rolls, roll_total);

    if let Err(why) = msg.reply(ctx, response).await {
        println!(
            "An error occurred while processing {:}roll: {:?}",
            BOT_PREFIX, why
        );
    }
    Ok(())
}
