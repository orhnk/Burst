mod client;
mod commands;
mod data;
mod error_handler;
mod handlers;
mod types;
mod utils;

use env_logger as logger;
use types::MaybeError;

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> MaybeError {
    dotenv::dotenv();

    logger::init_from_env("LOG_LEVEL");

    client::run().await
}
