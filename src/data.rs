use std::{
    str::FromStr as _,
    time::Duration,
};

use log::LevelFilter;
use poise::serenity_prelude::Color;
use sqlx::{
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    ConnectOptions as _,
    SqlitePool,
};

use crate::{
    types::Error,
    utils::{
        color_from_env,
        string_from_env,
    },
};

#[derive(Debug)]
pub struct Colors {
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            success: color_from_env("SUCCESS_COLOR"),
            error: color_from_env("ERROR_COLOR"),
            warning: color_from_env("WARNING_COLOR"),
            info: color_from_env("INFO_COLOR"),
        }
    }
}

#[derive(Debug)]
pub struct Emotes {
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
}

impl Default for Emotes {
    fn default() -> Self {
        Self {
            success: string_from_env("SUCCESS_EMOTE"),
            error: string_from_env("ERROR_EMOTE"),
            warning: string_from_env("WARNING_EMOTE"),
            info: string_from_env("INFO_EMOTE"),
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub db: SqlitePool,
    pub colors: Colors,
    pub emotes: Emotes,
    pub default_prefix: String,
}

impl Data {
    pub async fn default() -> Result<Self, Error> {
        let mut db_options =
            SqliteConnectOptions::from_str(string_from_env("DATABASE_URL").as_str())?
                .create_if_missing(true);

        let db_options = db_options
            .log_statements(LevelFilter::Debug)
            // TODO: Fine tune the duration.
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

        let db_pool = SqlitePoolOptions::default()
            .max_connections(100)
            .connect_with(db_options.clone())
            .await?;

        let default_prefix = string_from_env("DEFAULT_PREFIX");

        // Create the tables if they don't exist.
        // TODO: Make this work.
        //{
        //    sqlx::query!(
        //        r#"image.png
        //        CREATE TABLE IF NOT EXISTS prefixes (
        //            id INTEGER NOT NULL PRIMARY KEY,
        //            prefix TEXT NOT NULL DEFAULT ?
        //        )
        //        "#,
        //        default_prefix,
        //    )
        //    .execute(&db_pool)
        //    .await?;
        //}

        Ok(Self {
            db: db_pool,
            colors: Default::default(),
            emotes: Default::default(),
            default_prefix,
        })
    }
}
