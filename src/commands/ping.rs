use crate::{Context, Error};

/// Pong!
///
/// Pong!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    // poise::say_reply(ctx, "pong!".to_string()).await?;
    ctx.reply("pong!".to_string()).await?;
    Ok(())
}
