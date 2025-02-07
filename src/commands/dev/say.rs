use crate::{Context, Error};

#[poise::command(prefix_command, owners_only)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "the message to send"]
    #[rest]
    message: String,
) -> Result<(), Error> {
    let Context::Prefix(prefix_ctx) = ctx else {
        unreachable!("this command can only be ran through a prefix so this should never happen")
    };

    match prefix_ctx.msg.delete(ctx).await {
        Ok(_) => tracing::info!("Successfully deleted a message that invoked the `say` command"),
        Err(e) => tracing::warn!(
            "Failed to delete a message that invoked the 'say' command: {}",
            e
        ),
    }

    if let Err(e) = ctx.say(message).await {
        tracing::warn!(
            "Failed to send a message while performing the 'say' command: {}",
            e
        );
    }

    Ok(())
}
