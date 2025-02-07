use songbird::input::YoutubeDl;

use crate::{Context, Error};

/// Joins a voice channel
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let guild_id = guild.id;
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild_id, channel_id)
    };

    let c = match channel_id {
        None => {
            ctx.reply("You are not in a voice channel.").await?;
            return Ok(());
        }
        Some(c) => c,
    };

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    manager.join(guild_id, c).await?;

    Ok(())
}

/// Leaves a voice channel
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    match manager.remove(guild_id).await {
        Ok(_) => (),
        Err(e) => match e {
            songbird::error::JoinError::Dropped => todo!(),
            songbird::error::JoinError::NoSender => todo!(),
            songbird::error::JoinError::NoCall => {
                ctx.reply("I am not in a voice channel.").await?;
                return Ok(());
            }
            songbird::error::JoinError::TimedOut => todo!(),
            songbird::error::JoinError::Driver(error) => todo!(),
            songbird::error::JoinError::Serenity(try_send_error) => todo!(),
            _ => todo!(),
        },
    };

    Ok(())
}

/// Mutes the bot
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let handler = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("I am not in a voice channel.").await?;
            return Ok(());
        }
    };

    let mut handler = handler.lock().await;

    if handler.is_mute() {
        ctx.reply("Already muted").await?;
    } else {
        handler.mute(true).await?;
    }
    Ok(())
}

/// Unmutes the bot
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn unmute(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let handler = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("I am not in a voice channel.").await?;
            return Ok(());
        }
    };

    let mut handler = handler.lock().await;

    if !handler.is_mute() {
        ctx.reply("Already unmuted").await?;
    } else {
        handler.mute(false).await?;
    }
    Ok(())
}

/// Deafens the bot
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let handler = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("I am not in a voice channel.").await?;
            return Ok(());
        }
    };

    let mut handler = handler.lock().await;

    if handler.is_deaf() {
        ctx.reply("Already deafened").await?;
    } else {
        handler.deafen(true).await?;
    }
    Ok(())
}

/// Undeafens the bot
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn undeafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let handler = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("I am not in a voice channel.").await?;
            return Ok(());
        }
    };

    let mut handler = handler.lock().await;

    if !handler.is_deaf() {
        ctx.reply("Already undeafened").await?;
    } else {
        handler.deafen(false).await?;
    }
    Ok(())
}

/// Plays a song
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "the link of the song to play"]
    #[rest]
    song: String,
) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let guild_id = guild.id;
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild_id, channel_id)
    };

    let search_for_song = !song.starts_with("http");

    let http_client = ctx.data().http.clone();

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    manager.join(guild_id, channel_id.unwrap()).await?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if search_for_song {
            YoutubeDl::new_search(http_client, song)
        } else {
            YoutubeDl::new(http_client, song)
        };

        if handler.queue().current().is_some() {
            let th = handler.enqueue(src.clone().into()).await;
            let info = th.get_info().await?;
            ctx.reply(format!("Queued: {:?}", info)).await?;
        } else {
            let th = handler.enqueue(src.clone().into()).await;
            let info = th.get_info().await?;
            ctx.reply(format!("Now playing: {:?}", info)).await?;
        }
    }

    Ok(())
}

/// Stop playing
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("guild only command");

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let _ = handler.stop();
    }

    if manager.get(guild_id).is_some() {
        let _ = manager.leave(guild_id).await;
    }

    Ok(())
}

/// Skips the playing song
#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("guild only command");

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let _ = handler.queue().skip();
    }

    Ok(())
}
