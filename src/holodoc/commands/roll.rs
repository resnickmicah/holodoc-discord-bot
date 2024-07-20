use std::convert::TryInto;
use serenity::model::id::GuildId;

use super::*;

/// roll with 1d20+6, 1d20-3, 2d6, etc.
#[poise::command(slash_command, aliases("rx"))]
pub async fn roll(
    ctx: Context<'_>,
    roll_expr: String,
) -> Result<(), Error> {
    let mut num_dice: Option<u16> = None;
    let mut num_sides: Option<u16> = None;
    let mut modifier: Option<i16> = None;
    let mut rest: Option<&str> = None;
    if let Some((nd, rst)) = roll_expr.split_once('d') {
        num_dice = Some(str::parse::<u16>(nd).unwrap_or(0));
        rest = Some(rst);
        if num_dice == Some(0) {
            ctx.say(format!(
                "Invalid number of dice: {} from '{}'",
                nd,
                roll_expr
            )).await?;
            return Ok(());
        }
    } else {
        ctx.say(format!(
            "Missing 'd' from roll expression: {}",
            roll_expr
        )).await?;
        return Ok(());
    }

    if let Some((ns, pos_mod)) = rest.unwrap().split_once('+') {
        num_sides = Some(str::parse::<u16>(ns).unwrap_or(0));
        if num_sides == Some(0) {
            ctx.say(format!(
                "Invalid number of sides: {} from '{}'",
                ns,
                roll_expr
            )).await?;
            return Ok(()); 
        }
        if pos_mod.len() > 0 {
            modifier = Some(str::parse::<i16>(pos_mod).unwrap_or(0));
            if modifier == Some(0) {
                ctx.say(format!(
                    "Invalid modifier: {} from '{}'",
                    pos_mod,
                    roll_expr
                )).await?;
                return Ok(());
            }
        }
    } else if let Some((ns, neg_mod)) = rest.unwrap().split_once('-') {
        num_sides = Some(str::parse::<u16>(ns).unwrap_or(0));
        if num_sides == Some(0) {
            ctx.say(format!(
                "Invalid number of sides: {} from '{}'",
                ns,
                roll_expr
            )).await?;
            return Ok(());
        }
        if neg_mod.len() > 0 {
            modifier = Some(str::parse::<i16>(neg_mod).unwrap_or(0) * -1);
            if modifier == Some(0) {
                ctx.say(format!(
                    "Invalid modifier: {} from '{}'",
                    neg_mod,
                    roll_expr
                )).await?;
                return Ok(());
            }
        }
    } else {
        num_sides = Some(str::parse::<u16>(rest.unwrap()).unwrap_or(0));
        if num_sides == Some(0) {
            ctx.say(format!(
                "Invalid number of sides: {} from '{}'",
                rest.unwrap(),
                roll_expr
            )).await?;
            return Ok(());
        }
    }
    let roll_result = perform_roll(num_dice.unwrap(), num_sides.unwrap(), modifier);
    // let user_name = (&ctx.author().name).to_string();
    let user_nick = ctx.author();
    let response = format!("{} rolled {}:\n{}", user_nick, roll_expr, roll_result);
    ctx.say(response).await?;
    Ok(())
}

/// roll e.g. with 1d6+7, num_dice = 1, num_sides = 6, bonus = 7
#[poise::command(slash_command)]
pub async fn rparams(
    ctx: Context<'_>,
    num_dice: u16,
    num_sides: u16,
    modifier: Option<i16>,
) -> Result<(), Error> {
    let response = perform_roll(num_dice, num_sides, modifier);

    ctx.say(response).await?;
    Ok(())
}

pub fn perform_roll(num_dice: u16, num_sides: u16, modifier: Option<i16>) -> String {
    let mut rolls: Vec<i16> = vec![];
    let mut roll_total: i16 = 0;

    if num_dice == 0 || num_sides == 0 {
        return format!(
            "Invalid params: num_dice={} num_sides={}",
            num_dice, num_sides
        );
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
            ":headstone: **Crit fail!!** "
        }
        _ if roll_total == 20 && num_sides == 20 && num_dice == 1 => {
            ":crab::tada::sparkles: **Nat 20 bby!!**  :sparkles::tada::crab: "
        }
        (i16::MIN..=0) if num_sides == 20 && num_dice == 1 => {
            ":scream::headstone: **__¡¡FAILtacular!!__** "
        }
        (20..=39) if num_sides == 20 && num_dice == 1 => {
            ":tada::sparkles: **Gurēto success, very naisu!** "
        }
        (40..=i16::MAX) if num_sides == 20 && num_dice == 1 => {
            ":exploding_head::star_struck: **__GODLIKE!__** "
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
    return response;
}
