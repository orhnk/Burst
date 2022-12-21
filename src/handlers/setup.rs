use std::time::Duration;

use log::LevelFilter;
use poise::{
    serenity_prelude::{
        Activity,
        Context,
        Ready,
    },
    BoxFuture,
    Framework,
};
use sqlx::{
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    ConnectOptions,
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
    ctx.set_activity(Activity::listening("to music")).await;

    let mut db_options = SqliteConnectOptions::default()
        .filename("burst.db")
        .create_if_missing(true);

    let db_options = db_options
        .log_statements(LevelFilter::Debug)
        // TODO: Fine tune the duration.
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

    let db_pool = SqlitePoolOptions::default()
        .max_connections(100)
        .connect_with(db_options.clone())
        .await?;

    Ok(Data { db: db_pool })
}

pub fn handler<'a>(
    ctx: &'a Context,
    ready: &'a Ready,
    framework: &'a Framework<Data, Error>,
) -> BoxFuture<'a, Result<Data, Error>> {
    Box::pin(handle(ctx, ready, framework))
}
