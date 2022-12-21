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
    // TODO: Replace this with a real ping, idk how.
    ctx.say(format!("Pong! `{}ms`", 123)).await?;
    Ok(())
}
