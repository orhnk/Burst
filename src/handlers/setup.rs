use poise::{
    serenity_prelude::{
        Activity,
        Context,
        Ready,
    },
    BoxFuture,
    Framework,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(
    ctx: &Context
) -> Result<Data, Error> {
    ctx.set_activity(Activity::listening("music")).await;

    Data::default().await
}

pub fn handler<'a>(
    ctx: &'a Context,
    _ready: &'a Ready,
    _framework: &'a Framework<Data, Error>,
) -> BoxFuture<'a, Result<Data, Error>> {
    Box::pin(handle(ctx))
}
