use std::{
    sync::Arc,
    time::Duration,
};

use poise::{
    serenity_prelude::{
        CreateAllowedMentions as AllowedMentions,
        GatewayIntents as Intents,
    },
    EditTracker,
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

use crate::{
    commands::commands,
    data::Data,
    error_handler::error_handler,
    handlers,
    types::{
        Error,
        MaybeError,
    },
    utils::string_from_env,
};

#[inline]
fn prefix_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        dynamic_prefix: Some(handlers::dynamic_prefix_handler),
        edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(60))),
        ..Default::default()
    }
}

#[inline]
fn framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: commands(),
        prefix_options: prefix_options(),
        skip_checks_for_owners: true,
        allowed_mentions: Some({
            let mut allowed_mentions = AllowedMentions::default();
            allowed_mentions.replied_user(false);
            allowed_mentions
        }),
        event_handler: handlers::event_handler,
        on_error: error_handler,
        pre_command: handlers::pre_command_handler,
        post_command: handlers::post_command_handler,
        ..Default::default()
    }
}

#[inline]
async fn client() -> Result<Arc<Framework<Data, Error>>, Error> {
    let builder = Framework::builder()
        .token(string_from_env("BOT_TOKEN"))
        .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .options(framework_options())
        .setup(handlers::setup_handler);

    Ok(builder.build().await?)
}

pub async fn run() -> MaybeError {
    client().await?.start_autosharded().await?;

    Ok(())
}
