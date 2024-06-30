use super::{DEVELOPER_GUILDS, DEVELOPER_USER_IDS};
use crate::{Context, Error};
use poise::send_reply;

#[poise::command(prefix_command)]
pub async fn devregister(ctx: Context<'_>) -> Result<(), Error> {
    if DEVELOPER_USER_IDS.contains(&ctx.author().id) {
        if ctx
            .guild_id()
            .is_some_and(|id| DEVELOPER_GUILDS.contains(&id))
        {
            poise::builtins::register_application_commands_buttons(ctx).await?;
        } else {
            ctx.reply("Rerun the command in a testing guild.").await?;
        }
    } else {
        send_reply(
            ctx,
            poise::CreateReply::default()
                .content("Insufficient permissions.")
                .ephemeral(true),
        )
        .await?;
    }
    Ok(())
}
