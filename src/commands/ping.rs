use std::time;

use poise::{
    command,
    Context,
};

use crate::{
    data::Data,
    types::Error,
};

#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_, Data, Error>) -> Result<(), Error> {
    let color = ctx.data().colors.info;
    let emote = &ctx.data().emotes.info;

    let before = time::Instant::now();

    let message = ctx
        .send(|builder| {
            builder.reply(true);
            builder.embed(|embed| {
                embed.color(color);
                embed.title(format!("{} Pong!", emote))
            })
        })
        .await?;

    let after = time::Instant::now();

    message
        .edit(ctx, |builder| {
            builder.embed(|embed| {
                embed.color(color);
                embed.title(format!(
                    "{} Pong! `{}ms`",
                    emote,
                    after.duration_since(before).as_millis()
                ))
            })
        })
        .await?;

    Ok(())
}
