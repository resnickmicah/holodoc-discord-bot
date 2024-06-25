use super::*;

/// roll e.g. with 1d6+7, num_dice = 1, num_sides = 6, bonus = 7
#[poise::command(slash_command, aliases("r"))]
pub async fn roll(
    ctx: Context<'_>,
    num_dice: u16,
    num_sides: u16,
    modifier: Option<i16>,
) -> Result<(), Error> {
    let mut rolls: Vec<u16> = vec![];
    let mut roll_total: u16 = 0;
    for _ in 1..=num_dice {
        let mut rng = thread_rng();
        let roll_result: u16 = rng.gen_range(1..=num_sides);
        rolls.push(roll_result);
        roll_total += roll_result;
    }
    let modifier_value = modifier.unwrap_or(0);
    roll_total = roll_total.checked_add_signed(modifier_value).unwrap_or(1);

    let response = if modifier_value != 0 {
        let modifier_str: String = if modifier_value > 0 {
            format!("+ {}", modifier_value)
        } else {
            format!("- {}", modifier_value.abs())
        };
        format!(
            "Roll results: {:?} {}, total: {}",
            rolls, modifier_str, roll_total
        )
    } else {
        format!("Roll results: {:?}, total: {}", rolls, roll_total)
    };

    ctx.say(response).await?;
    Ok(())
}
