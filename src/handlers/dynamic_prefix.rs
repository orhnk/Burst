use poise::{
    BoxFuture,
    PartialContext,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(_ctx: PartialContext<'_, Data, Error>) -> Result<Option<String>, Error> {
    // TODO
    Ok(Some(">".to_owned()))
}

pub fn handler(
    ctx: PartialContext<'_, Data, Error>,
) -> BoxFuture<'_, Result<Option<String>, Error>> {
    Box::pin(handle(ctx))
}
