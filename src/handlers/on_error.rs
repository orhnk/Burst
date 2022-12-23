use log::error;
use poise::{
    BoxFuture,
    FrameworkError,
};

use crate::{
    data::Data,
    types::Error,
};

#[allow(clippy::match_single_binding)] // TODO: Remove this when the error handler is implemented.
async fn handle(error: FrameworkError<'_, Data, Error>) {
    if let Some(ctx) = error.ctx() {
        match error {
            _ => {
                error!(
                    "An error has occured while executing the command {}",
                    ctx.command().qualified_name
                );
            },
        }
    }
    else {
        error!("An uncaught error has occured: {error:?}");
    }
}

pub fn handler(error: FrameworkError<'_, Data, Error>) -> BoxFuture<'_, ()> {
    Box::pin(handle(error))
}
