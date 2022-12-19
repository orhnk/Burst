use std::{
    collections::HashSet,
    env,
    error,
    sync::Arc,
    time::Duration,
};

use handlers::{
    event,
    on_error,
    post_command,
    pre_command,
};
use poise::{
    serenity_prelude::{
        Activity,
        GatewayIntents as Intents,
    },
    EditTracker,
    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

type Error = Box<dyn error::Error + Send + Sync>;

pub async fn run() -> Result<(), Error> {
    let log_webhook_url = env::var("LOG_WEBHOOK_URL")?;

    Framework::builder()
        .token(env::var("TOKEN")?)
        .intents(Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .setup(move |ctx, _, _| {
            Box::pin(async move {
                ctx.set_activity(Activity::listening("to Music"));
                Ok(())
            })
        })
        .options(FrameworkOptions {
            commands: vec![],
            on_error: on_error::on_error,
            pre_command: pre_command::handler,
            post_command: post_command::handler,
            command_check: None,
            skip_checks_for_owners: true,
            allowed_mentions: None,
            reply_callback: None,
            manual_cooldowns: false,
            require_cache_for_guild_check: false,
            event_handler: event::handler,
            #[allow(deprecated)]
            listener: (), // TODO
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(">".to_string()),
                additional_prefixes: vec![],
                dynamic_prefix: Some(|ctx| {
                    // TODO
                    Box::pin(async move { Ok(None) })
                }),
                stripped_dynamic_prefix: None,
                mention_as_prefix: true,
                edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(60))),
                execute_untracked_edits: true,
                ignore_edits_if_not_yet_responded: false,
                execute_self_messages: false,
                ignore_bots: true,
                case_insensitive_commands: true,
                __non_exhaustive: (),
            },
            owners: HashSet::new(),
            __non_exhaustive: (),
        })
        .build()
        .await?
        .start_autosharded()
        .await
}
