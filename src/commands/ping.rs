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
    ctx.say("Pong!").await?;
    Ok(())
}
