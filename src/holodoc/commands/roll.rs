use super::*;

/// roll [num dice] [num sides]
#[poise::command(slash_command, aliases("r"))]
pub async fn roll(ctx: Context<'_>, num_dice: u16, num_sides: u16) -> Result<(), Error> {
    let mut rolls: Vec<u16> = vec![];
    let mut roll_total: u16 = 0;
    for _ in 1..=num_dice {
        let mut rng = thread_rng();
        let roll_result: u16 = rng.gen_range(1..=num_sides);
        rolls.push(roll_result);
        roll_total += roll_result;
    }
    let response = format!("Roll results: {:?}, total: {}", rolls, roll_total);

    ctx.say(response).await?;
    Ok(())
}
