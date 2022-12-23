use log::info;
use poise::{
    BoxFuture,
    Context,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(ctx: Context<'_, Data, Error>) {
    let author = ctx.author();

    info!(
        "Command '{}' run started for caller {}#{} (ID: {}).",
        ctx.command().name,
        author.name,
        author.discriminator,
        author.id,
    )
}

pub fn handler(ctx: Context<'_, Data, Error>) -> BoxFuture<'_, ()> {
    Box::pin(handle(ctx))
}
