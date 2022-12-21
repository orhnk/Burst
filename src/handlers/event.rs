use poise::{
    serenity_prelude::Context,
    BoxFuture,
    Event,
    FrameworkContext,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(
    _ctx: &Context,
    _event: &Event<'_>,
    _framework: FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    Ok(())
}

pub fn handler<'a>(
    ctx: &'a Context,
    event: &'a Event<'a>,
    framework: FrameworkContext<'a, Data, Error>,
    _: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(handle(ctx, event, framework))
}
