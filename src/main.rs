mod client;
mod commands;
mod data;
mod handlers;
mod types;

use poise::serenity_prelude::Error as SerenityError;
use pretty_env_logger as logger;

#[tokio::main]
async fn main() -> Result<(), SerenityError> {
    let _ = dotenv::dotenv();

    logger::init_custom_env("LOG_LEVEL");

    client::run().await
}
