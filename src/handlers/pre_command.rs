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
        "{}#{} (ID: {}) ran command '{}'.",
        author.name,
        author.discriminator,
        author.id,
        ctx.command().name
    )
}

pub fn handler<'a>(ctx: Context<'a, Data, Error>) -> BoxFuture<'a, ()> {
    Box::pin(handle(ctx))
}
