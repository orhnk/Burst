use poise::{
    BoxFuture,
    PartialContext,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(ctx: PartialContext<'_, Data, Error>) -> Result<Option<String>, Error> {
    // TODO
    Ok(Some(ctx.data.default_prefix.clone()))
}

pub fn handler(
    ctx: PartialContext<'_, Data, Error>,
) -> BoxFuture<'_, Result<Option<String>, Error>> {
    Box::pin(handle(ctx))
}
