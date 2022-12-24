mod client;
mod commands;
mod data;
mod handlers;
mod types;

use env_logger as logger;
use types::MaybeError;

#[allow(unused_must_use)]
#[tokio::main]
async fn main() -> MaybeError {
    dotenv::dotenv();

    logger::init_from_env("LOG_LEVEL");

    client::run().await
}
