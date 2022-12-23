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
    ctx: &Context,
    _ready: &Ready,
    _framework: &Framework<Data, Error>,
) -> Result<Data, Error> {
    ctx.set_activity(Activity::listening("music")).await;

    Ok(Data::default().await?)
}

pub fn handler<'a>(
    ctx: &'a Context,
    ready: &'a Ready,
    framework: &'a Framework<Data, Error>,
) -> BoxFuture<'a, Result<Data, Error>> {
    Box::pin(handle(ctx, ready, framework))
}
