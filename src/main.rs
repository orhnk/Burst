mod client;
mod commands;
mod data;
mod handlers;
mod types;

use poise::serenity_prelude::Error as SerenityError;
use pretty_env_logger as logger;

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), SerenityError> {
    dotenv::dotenv();

    logger::init_custom_env("LOG_LEVEL");

    client::run().await
}
