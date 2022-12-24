use poise::{
    serenity_prelude::Context,
    BoxFuture,
    Event,
};

use crate::{
    data::Data,
    types::{
        FrameworkContext,
        MaybeError,
    },
};

async fn handle(
    _ctx: &Context,
    _event: &Event<'_>,
    _framework: FrameworkContext<'_>,
) -> MaybeError {
    Ok(())
}

pub fn handler<'a>(
    ctx: &'a Context,
    event: &'a Event<'a>,
    framework: FrameworkContext<'a>,
    _: &'a Data,
) -> BoxFuture<'a, MaybeError> {
    Box::pin(handle(ctx, event, framework))
}
