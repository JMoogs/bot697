use crate::{Context, Error};

#[poise::command(prefix_command, owners_only)]
pub async fn devregister(ctx: Context<'_>) -> Result<(), Error> {
    if ctx
        .guild_id()
        .is_some_and(|id| ctx.data().developer_guilds.contains(&id))
    {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        ctx.reply("Rerun the command in a testing guild.").await?;
    }
    Ok(())
}
