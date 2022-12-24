use poise::command;

use crate::{
    types::{
        Context,
        MaybeError,
    },
    util::cut_excess,
};

async fn help_autocomplete<'a>(
    ctx: Context<'a>,
    query: &'a str,
) -> impl Iterator<Item = String> + 'a {
    ctx.framework()
        .options
        .commands
        .iter()
        .filter(move |&command| !command.hide_in_help && command.name.contains(query))
        .map(|command| command.name.to_string())
}

async fn help_specific_command(ctx: Context<'_>, command_name: String) -> MaybeError {
    let data = ctx.data();

    let command = ctx.framework().options.commands.iter().find(|&command| {
        !command.hide_in_help
            && (command
                .qualified_name
                .eq_ignore_ascii_case(command_name.as_str())
                || command
                    .context_menu_name
                    .unwrap_or("\0")
                    .eq_ignore_ascii_case(command_name.as_str()))
    });

    match command {
        None => {
            ctx.send(|builder| {
                builder.reply(true);
                builder.ephemeral(true);
                builder.embed(|embed| {
                    embed.color(data.colors.error);
                    embed.title(format!(
                        "{} Command `/{}` not found.",
                        data.emotes.error,
                        cut_excess(command_name.replace('`', ""), 32)
                    ))
                })
            })
            .await?;
        },

        Some(command) => {
            ctx.send(|builder| {
                builder.reply(true);
                builder.embed(|embed| {
                    embed.color(data.colors.info);

                    embed.title(format!(
                        "{} Showing help for `/{}`",
                        data.emotes.info, command.qualified_name
                    ));
                    // Description is mandatory.
                    embed.description(command.description.as_ref().unwrap());

                    embed.field("Category", command.category.unwrap(), true);

                    if !command.aliases.is_empty() {
                        embed.field(
                            "Aliases",
                            command
                                .aliases
                                .iter()
                                .map(|alias| format!("`/{alias}`"))
                                .collect::<Vec<_>>()
                                .join(", "),
                            true,
                        );
                    }

                    embed
                })
            })
            .await?;
        },
    }

    Ok(())
}

async fn help_all_commands(ctx: Context<'_>) -> MaybeError {
    // TODO
    ctx.say("not implemented yet lol cope").await?;

    Ok(())
}

/// Sends help about a specific command or all commands.
#[command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Miscellaneous"
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "The command to get help about. Leave blank if you want a list of all \
                     commands."]
    #[autocomplete = "help_autocomplete"]
    command: Option<String>,
) -> MaybeError {
    match command {
        Some(command) => help_specific_command(ctx, command).await?,
        None => help_all_commands(ctx).await?,
    }

    Ok(())
}
