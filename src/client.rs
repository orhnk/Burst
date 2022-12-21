use std::{
    env,
    time::Duration,
};

use poise::{
    serenity_prelude::{
        Error as SerenityError,
        GatewayIntents as Intents,
    },
    EditTracker,
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

use crate::{
    commands,
    data::Data,
    handlers::{
        dynamic_prefix,
        event,
        on_error,
        post_command,
        pre_command,
        setup,
    },
    types::Error,
};

fn prefix_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        dynamic_prefix: Some(dynamic_prefix::handler),
        edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(60))),
        ..Default::default()
    }
}

fn framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: commands::COMMANDS,
        prefix_options: prefix_options(),
        skip_checks_for_owners: true,
        allowed_mentions: None,
        event_handler: event::handler,
        on_error: on_error::handler,
        pre_command: pre_command::handler,
        post_command: post_command::handler,
        ..Default::default()
    }
}

pub async fn run() -> Result<(), SerenityError> {
    let builder = Framework::builder()
        .setup(setup::handler)
        .options(framework_options())
        .token(env::var("BOT_TOKEN").expect("BOT_TOKEN not set."))
        .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT);

    builder.build().await?.start_autosharded().await
}
