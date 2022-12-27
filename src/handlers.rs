use log::info;
use poise::{
    serenity_prelude::{
        Activity,
        Context as SerenityContext,
        Ready,
    },
    BoxFuture,
    Event,
    Framework,
};

use crate::{
    data::Data,
    types::{
        Context,
        Error,
        FrameworkContext,
        PartialContext,
    },
    utils::string_from_env,
};

pub fn dynamic_prefix_handler(
    ctx: PartialContext<'_>,
) -> BoxFuture<'_, Result<Option<String>, Error>> {
    Box::pin(async move {
        let default_prefix = || (ctx.data.default_prefix.clone(),);

        let prefix = if let Some(guild_id) = ctx.guild_id {
            sqlx::query_as::<_, (String,)>(r"SELECT prefix FROM prefixes WHERE id = ?")
                .bind(guild_id.0 as i64)
                .fetch_optional(&ctx.data.db)
                .await?
                .unwrap_or_else(default_prefix)
        }
        else {
            default_prefix()
        }
        .0;

        Ok(Some(prefix))
    })
}

pub fn event_handler<'a>(
    _ctx: &'a SerenityContext,
    _event: &'a Event<'a>,
    _framework_ctx: FrameworkContext<'a>,
    _data: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move {
        // TODO

        Ok(())
    })
}

pub fn pre_command_handler(ctx: Context<'_>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        let author = ctx.author();

        info!(
            "Command '{}' run started for caller {}#{} (ID: {}).",
            ctx.command().name,
            author.name,
            author.discriminator,
            author.id,
        )
    })
}

pub fn post_command_handler(ctx: Context<'_>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        let author = ctx.author();

        info!(
            "Command '{}' run finished for caller {}#{} (ID: {}).",
            ctx.command().name,
            author.name,
            author.discriminator,
            author.id,
        )
    })
}

pub fn setup_handler<'a>(
    ctx: &'a SerenityContext,
    _ready: &'a Ready,
    _framework: &'a Framework<Data, Error>,
) -> BoxFuture<'a, Result<Data, Error>> {
    Box::pin(async move {
        ctx.set_activity(Activity::listening(string_from_env("MOTD")))
            .await;

        Data::default().await
    })
}
