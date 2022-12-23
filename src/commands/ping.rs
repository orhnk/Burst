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
    let before = time::Instant::now();

    let message = ctx
        .send(|builder| builder.embed(|embed| embed.title("Pong!")))
        .await?;

    let after = time::Instant::now();

    message
        .edit(ctx, |builder| {
            builder.embed(|embed| {
                embed.title(format!(
                    "Pong! `{}ms`",
                    after.duration_since(before).as_millis()
                ))
            })
        })
        .await?;

    Ok(())
}
