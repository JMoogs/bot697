use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Kick a user
///
/// Kicks a user from a guild
#[poise::command(prefix_command, slash_command, guild_only, owners_only)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "the user to kick"] member: serenity::Member,
    #[description = "the reason for kicking them"]
    #[rest]
    reason: Option<String>,
) -> Result<(), Error> {
    let reason = reason.unwrap_or_else(|| String::from("no reason provided"));
    let res = member.kick_with_reason(&ctx.http(), &reason).await;

    match res {
        Err(e) => {
            tracing::info!(
                "Failed to kick a user: {} from guild: {} for reason: {}",
                member.user.id,
                member.guild_id,
                e
            );
            match ctx.reply(format!("Couldn't kick the user: {e}")).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::info!(
                        "Failed to notify user that kicking failed in guild: {} for reason: {}",
                        member.guild_id,
                        e
                    );
                }
            }
        }
        Ok(_) => {
            tracing::info!(
                "Kicked user: {} from guild: {} for reason: {}",
                member.user.id,
                member.guild_id,
                reason
            );
            let response = ctx
                .send(
                    poise::CreateReply::default().embed(
                        serenity::CreateEmbed::new()
                            .title(format!("{} was kicked", member.user.name))
                            .thumbnail(member.avatar_url().unwrap_or_else(|| {
                                member.user.avatar_url().unwrap_or_else(|| {
                                    String::from("https://cdn.discordapp.com/embed/avatars/0.png")
                                })
                            }))
                            .description(format!("The user was kicked for: {}", reason))
                            .timestamp(serenity::Timestamp::now())
                            .colour((255, 0, 0)),
                    ),
                )
                .await;
            match response {
                Ok(_) => (),
                Err(e) => {
                    tracing::info!(
                        "Failed to notify user that kicking succeeded in guild: {} for reason: {}",
                        member.guild_id,
                        e
                    );
                }
            }
        }
    };

    Ok(())
}
