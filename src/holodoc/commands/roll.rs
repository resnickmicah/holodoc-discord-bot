use std::convert::TryInto;

use super::*;

/// roll e.g. with 1d6+7, num_dice = 1, num_sides = 6, bonus = 7
#[poise::command(slash_command, aliases("r"))]
pub async fn roll(
    ctx: Context<'_>,
    num_dice: u16,
    num_sides: u16,
    #[rename = "+"] modifier: Option<i16>,
) -> Result<(), Error> {
    let mut rolls: Vec<i16> = vec![];
    let mut roll_total: i16 = 0;

    if num_dice == 0 || num_sides == 0 {
        ctx.say(format!(
            "Invalid params: num_dice={} num_sides={}",
            num_dice, num_sides
        ))
        .await?;
        return Ok(());
    }
    for _ in 1..=num_dice {
        let mut rng = thread_rng();
        let roll_result: i16 = rng.gen_range(1..=num_sides).try_into().unwrap();
        rolls.push(roll_result);
        roll_total += roll_result;
    }
    let modifier_value = modifier.unwrap_or(0);
    let modified_roll = roll_total + modifier_value;

    let flavor = match modified_roll {
        _ if roll_total == 1 && num_sides == 20 && num_dice == 1 => {
            ":facepalm: **Crit fail!!** => "
        }
        (i16::MIN..=0) if num_sides == 20 && num_dice == 1 => {
            ":scream::headstone: **__¡¡FAILtacular!!__** "
        }
        (20..=39) if num_sides == 20 && num_dice == 1 => {
            ":tada::sparkles: **Gurēto success, very naisu!** "
        }
        (40..=i16::MAX) if num_sides == 20 && num_dice == 1 => {
            ":exploding_head::star_struck: **__GODLIKE!__**"
        }
        _ => ":game_die: Roll results: ",
    };

    let response = if modifier_value != 0 {
        let modifier_str: String = if modifier_value > 0 {
            format!("+ {}", modifier_value)
        } else {
            format!("- {}", modifier_value.abs())
        };
        format!(
            "{} {:?} {}, total: {}",
            flavor, rolls, modifier_str, modified_roll
        )
    } else {
        format!("{} {:?}, total: {}", flavor, rolls, modified_roll)
    };

    ctx.say(response).await?;
    Ok(())
}
