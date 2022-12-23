use std::time::Duration;

use log::LevelFilter;
use sqlx::{
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    ConnectOptions as _,
    SqlitePool,
};

use crate::types::Error;

#[derive(Debug)]
pub struct Data {
    pub db: SqlitePool,
}

impl Data {
    pub async fn default() -> Result<Self, Error> {
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

        Ok(Self { db: db_pool })
    }
}
