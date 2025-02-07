use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Displays info about a user
#[poise::command(prefix_command, slash_command)]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "the user to show info about"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());

    let embed = serenity::CreateEmbed::default()
        .title(format!("{}'s info", user.display_name()))
        .thumbnail(
            user.avatar_url()
                .unwrap_or_else(|| user.default_avatar_url()),
        )
        .field("Username", user.name.clone(), true)
        .field("ID", user.id.to_string(), true)
        .field("Display name", user.display_name(), true)
        .field("Bot", user.bot.to_string(), true)
        .field("Created at", user.created_at().to_rfc2822(), true)
        .field("System user", user.system.to_string(), true);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Shows how long the bot has been running
#[poise::command(prefix_command, slash_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = ctx.data().start_time;

    let uptime = chrono::Duration::from_std(start_time.elapsed()).unwrap();
    let uptime = format!(
        "{:02}:{:02}:{:02}",
        uptime.num_hours(),
        uptime.num_minutes() % 60,
        uptime.num_seconds() % 60
    );

    ctx.send(poise::CreateReply::default().content(format!("I've been running for: {}", uptime)))
        .await?;

    Ok(())
}

/// Shows the avatar of a user
#[poise::command(prefix_command, slash_command)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "the user to show the avatar of"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());
    let avatar_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("{}'s avatar", user.display_name()))
                .image(avatar_url),
        ),
    )
    .await?;

    Ok(())
}

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
