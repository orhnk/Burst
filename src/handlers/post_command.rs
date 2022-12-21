use poise::{
    BoxFuture,
    Context,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(_ctx: Context<'_, Data, Error>) {
    // TODO
}

pub fn handler<'a>(ctx: Context<'a, Data, Error>) -> BoxFuture<'a, ()> {
    Box::pin(handle(ctx))
}
