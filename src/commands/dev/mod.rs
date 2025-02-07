use poise::serenity_prelude;

use crate::{Context, Error};

pub mod register;
pub mod say;

#[poise::command(prefix_command, owners_only)]
pub async fn dumpconfig(ctx: Context<'_>) -> Result<(), Error> {
    let config = ctx.data().config.clone();
    let config = serde_json::to_string_pretty(&config).unwrap();
    if config.len() > 1950 {
        ctx.send(
            poise::CreateReply::default()
                .content("The config is too long, sending as attachment")
                .attachment(serenity_prelude::CreateAttachment::bytes(
                    config.as_bytes(),
                    "config.json",
                )),
        )
        .await?;
    } else {
        ctx.reply(format!("```json\n{}\n```", config)).await?;
    }
    Ok(())
}
