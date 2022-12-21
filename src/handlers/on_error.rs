use poise::{
    BoxFuture,
    FrameworkError,
};

use crate::{
    data::Data,
    types::Error,
};

async fn handle(error: FrameworkError<'_, Data, Error>) {
    // TODO
}

pub fn handler<'a>(error: FrameworkError<'a, Data, Error>) -> BoxFuture<'a, ()> {
    Box::pin(handle(error))
}
