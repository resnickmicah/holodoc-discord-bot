use std::convert::TryFrom;

use errors::HolodocErrors;

use super::*;

#[derive(Debug)]
struct RollExpression {
    num_dice: u16,
    num_sides: u16,
    modifier: i16,
}

impl std::fmt::Display for RollExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let signed_modf = if self.modifier > 0 {
            format!("+{}", self.modifier)
        } else if self.modifier < 0 {
            format!("+{}", self.modifier.abs())
        } else {
            "".to_string()
        };
        write!(f, "{}d{}{}", self.num_dice, self.num_sides, signed_modf)
    }
}

impl TryFrom<&str> for RollExpression {
    type Error = HolodocErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(&['d', '+', '-'][..]).collect::<Vec<&str>>();
        println!("Split roll parts {:?}", parts);
        if parts.len() != 2 && parts.len() != 3 {
            return Err(HolodocErrors::RollExprFormatError(value.to_string()));
        }
        let num_dice: u16 = str::parse::<u16>(parts[0])
            .map_err(|_| HolodocErrors::RollExprValueError("number of dice".to_string(), parts[0].to_string()))?;
        let num_sides: u16 = str::parse::<u16>(parts[1])
            .map_err(|_| HolodocErrors::RollExprValueError("number of sides".to_string(), parts[1].to_string()))?;
        let modifier: i16 = if parts.len() == 3 {
            let mod_sign = if value.contains('-') { -1 } else { 1 };
            let mod_value = str::parse::<i16>(parts[2])
                .map_err(|_| HolodocErrors::RollExprValueError("modifier".to_string(), parts[2].to_string()))?;
            mod_value * mod_sign
        } else {
            0
        };
        Ok(RollExpression {
            num_dice,
            num_sides,
            modifier,
        })
    }
}

/// roll with 1d20+6, 1d20-3, 2d6, etc.
#[poise::command(slash_command, aliases("rx"))]
pub async fn roll(ctx: Context<'_>, roll_expr: String) -> Result<(), Error> {
    match RollExpression::try_from(roll_expr.as_str()) {
        Ok(rexp) => {
            let roll_result = perform_roll(&rexp);
            let user_nick = ctx.author();
            let response = format!("{} rolled {}:\n{}", user_nick, rexp, roll_result);
            ctx.say(response).await?;
            Ok(())
        }
        Err(rexp_err) => {
            ctx.say(rexp_err.to_string()).await?;
            Ok(())
        }
    }
}

fn perform_roll(rexpr: &RollExpression) -> String {
    let mut rolls: Vec<i16> = vec![];
    let mut roll_total: i16 = 0;

    for _ in 1..=rexpr.num_dice {
        let mut rng = thread_rng();
        let roll_result: i16 = rng.gen_range(1..=rexpr.num_sides).try_into().unwrap();
        rolls.push(roll_result);
        roll_total += roll_result;
    }
    let modified_roll = roll_total + rexpr.modifier;

    let flavor = match modified_roll {
        _ if roll_total == 1 && rexpr.num_sides == 20 && rexpr.num_dice == 1 => {
            ":headstone: **Crit fail!!** "
        }
        _ if roll_total == 20 && rexpr.num_sides == 20 && rexpr.num_dice == 1 => {
            ":crab::tada::sparkles: **Nat 20 bby!!**  :sparkles::tada::crab: "
        }
        (i16::MIN..=0) if rexpr.num_sides == 20 && rexpr.num_dice == 1 => {
            ":scream::headstone: **__¡¡FAILtacular!!__** "
        }
        (20..=39) if rexpr.num_sides == 20 && rexpr.num_dice == 1 => {
            ":tada::sparkles: **Gurēto success, very naisu!** "
        }
        (40..=i16::MAX) if rexpr.num_sides == 20 && rexpr.num_dice == 1 => {
            ":exploding_head::star_struck: **__GODLIKE!__** "
        }
        _ => ":game_die: Roll results: ",
    };

    let response = if rexpr.modifier != 0 {
        let modifier_str: String = if rexpr.modifier > 0 {
            format!("+ {}", rexpr.modifier)
        } else {
            format!("- {}", rexpr.modifier.abs())
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
