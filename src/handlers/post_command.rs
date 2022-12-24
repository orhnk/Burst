use log::info;
use poise::BoxFuture;

use crate::types::Context;

async fn handle(ctx: Context<'_>) {
    let author = ctx.author();

    info!(
        "Command '{}' run ended for caller {}#{} (ID: {}).",
        ctx.command().name,
        author.name,
        author.discriminator,
        author.id,
    )
}

pub fn handler(ctx: Context<'_>) -> BoxFuture<'_, ()> {
    Box::pin(handle(ctx))
}
