mod client;
mod handlers;

use std::error;

use pretty_env_logger as logger;

type Error = Box<dyn error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv::dotenv();

    logger::init_custom_env("LOG_LEVEL");

    client::run().await
}
