use crate::{Context, Error};
use rand::Rng;

/// Flip a coin
///
/// Simulates flipping a coin, landing on either heads or tails
#[poise::command(slash_command, prefix_command, aliases("cf"))]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let res: bool = rand::thread_rng().gen();

    if res {
        ctx.reply("The coin landed on **heads**.").await?;
    } else {
        ctx.reply("The coin landed on **tails**.").await?;
    }
    Ok(())
}

/// Rolls a 6-sided die
///
/// Rolls a 6-sided die
#[poise::command(slash_command, prefix_command, aliases("d6"))]
pub async fn dice(ctx: Context<'_>) -> Result<(), Error> {
    let num = rand::thread_rng().gen_range(1..=6);
    let msg = format!("You rolled a {}.", num);
    ctx.reply(msg).await?;
    Ok(())
}

/// Helps you make a decision
#[allow(unused_variables)]
#[poise::command(
    slash_command,
    prefix_command,
    aliases("8b"),
    rename = "8ball",
    discard_spare_arguments
)]
pub async fn ball8(ctx: Context<'_>, #[rest] question: Option<String>) -> Result<(), Error> {
    let num = rand::thread_rng().gen_range(1..=25);
    let msg = match num {
        1 | 2 => "Don't count on it",
        3 | 4 => "My reply is no",
        5 | 6 => "My sources say no",
        7 | 8 => "Outlook not so good",
        9 | 10 => "Very doubtful",
        11 => "Reply hazy, try again",
        12 => "Ask again later",
        13 => "Better not tell you now",
        14 => "Cannot predict now",
        15 => "Concentrate and ask again",
        16 => "It is certain",
        17 => "It is decidedly so",
        18 => "Without a doubt",
        19 => "Yes definitely",
        20 => "You may rely on it",
        21 => "As I see it, yes",
        22 => "Most likely",
        23 => "Outlook good",
        24 => "Yes",
        25 => "Signs point to yes",
        _ => unreachable!(),
    };
    ctx.reply(msg).await?;
    Ok(())
}
