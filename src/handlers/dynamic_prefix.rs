use poise::BoxFuture;

use crate::types::{
    Error,
    PartialContext,
};

type Result = std::result::Result<Option<String>, Error>;

async fn handle(ctx: PartialContext<'_>) -> Result {
    // TODO
    Ok(Some(ctx.data.default_prefix.clone()))
}

pub fn handler(ctx: PartialContext<'_>) -> BoxFuture<'_, Result> {
    Box::pin(handle(ctx))
}
