use log::error;
use poise::BoxFuture;

use crate::types::{
    Context,
    FrameworkError,
};

#[allow(unused_must_use)]
async fn uncaught_error(ctx: Context<'_>, error: FrameworkError<'_>) {
    let data = ctx.data();

    error!(
        "An uncaught error has occured while executing the command {}: {error:?}",
        ctx.command().qualified_name
    );

    ctx.send(|builder| {
        builder.reply(true);
        builder.ephemeral(true);
        builder.embed(|embed| {
            embed.color(data.colors.error);
            embed.title(format!(
                "{} An unknown error has occured",
                data.emotes.error
            ));
            embed.description("This incident will be reported.")
        })
    })
    .await;
}

#[allow(unused_must_use)]
async fn handle(error: FrameworkError<'_>) {
    let ctx = error.ctx();

    if ctx.is_none() {
        match error {
            FrameworkError::UnknownCommand { .. } => {},

            _ => error!("An uncaught error has occured: {error:?}"),
        }
        return;
    }

    let ctx = ctx.unwrap();

    match error {
        FrameworkError::ArgumentParse { input, .. } => {
            ctx.send(|builder| {
                builder.reply(true);
                builder.ephemeral(true);
                builder.embed(|embed| {
                    embed.color(ctx.data().colors.error);
                    embed.title(format!(
                        "{} Invalid arguments provided",
                        ctx.data().emotes.error
                    ));

                    if let Some(input) = input {
                        embed.description(format!("Invalid argument: {input}"));
                    }

                    embed
                })
            })
            .await;
        },

        FrameworkError::NotAnOwner { .. } => {},
        
        _ => uncaught_error(ctx, error).await,
    }
}

pub fn handler(error: FrameworkError<'_>) -> BoxFuture<'_, ()> {
    Box::pin(handle(error))
}
