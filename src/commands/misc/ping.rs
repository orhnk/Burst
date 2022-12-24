use std::time;

use poise::command;

use crate::types::{
    Context,
    MaybeError,
};

/// Sends the latency between the bot and Discord.
#[command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous",
    aliases("latency")
)]
pub async fn ping(ctx: Context<'_>) -> MaybeError {
    let color = ctx.data().colors.info;
    let emote = &ctx.data().emotes.info;

    let before = time::Instant::now();

    let message = ctx
        .send(|builder| {
            builder.reply(true);
            builder.embed(|embed| {
                embed.color(color);
                embed.title(format!("{emote} Pong!"))
            })
        })
        .await?;

    let after = time::Instant::now();

    message
        .edit(ctx, |builder| {
            builder.embed(|embed| {
                embed.color(color);
                embed.title(format!(
                    "{emote} Pong! `{}ms`",
                    after.duration_since(before).as_millis()
                ))
            })
        })
        .await?;

    Ok(())
}
