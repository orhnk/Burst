use std::{
    env,
    process::abort,
    time::Duration,
};

use log::error;
use poise::{
    serenity_prelude::GatewayIntents as Intents,
    EditTracker,
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

use crate::{
    commands::commands,
    data::Data,
    handlers,
    types::Error,
};

fn prefix_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        dynamic_prefix: Some(handlers::dynamic_prefix),
        edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(60))),
        ..Default::default()
    }
}

fn framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: commands(),
        prefix_options: prefix_options(),
        skip_checks_for_owners: true,
        allowed_mentions: None,
        event_handler: handlers::event,
        on_error: handlers::on_error,
        pre_command: handlers::pre_command,
        post_command: handlers::post_command,
        ..Default::default()
    }
}

pub async fn run() -> Result<(), Error> {
    let builder = Framework::builder()
        .setup(handlers::setup)
        .options(framework_options())
        .token(env::var("BOT_TOKEN").unwrap_or_else(|_| {
            error!("Expected the BOT_TOKEN environment variable to be set.");
            abort();
        }))
        .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT);

    builder.build().await?.start_autosharded().await?;

    Ok(())
}
