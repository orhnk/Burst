mod client;
mod commands;
mod data;
mod handlers;
mod types;

use pretty_env_logger as logger;
use types::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv::dotenv();

    logger::init_custom_env("LOG_LEVEL");

    client::run().await
}
