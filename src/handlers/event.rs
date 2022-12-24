use poise::{
    serenity_prelude::Context,
    BoxFuture,
    Event,
};

use crate::{
    data::Data,
    types::{
        Error,
        FrameworkContext,
    },
};

async fn handle(
    _ctx: &Context,
    _event: &Event<'_>,
    _framework: FrameworkContext<'_>,
) -> Result<(), Error> {
    Ok(())
}

pub fn handler<'a>(
    ctx: &'a Context,
    event: &'a Event<'a>,
    framework: FrameworkContext<'a>,
    _: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(handle(ctx, event, framework))
}
