use poise::{
    builtins::register_application_commands_buttons as poise_register_application_commands_buttons,
    command,
};

use crate::types::{
    Context,
    MaybeError,
};

/// Syncs the bots app commands globally or for the current guild.
#[command(
    prefix_command,
    slash_command,
    track_edits,
    broadcast_typing,
    category = "Owner",
    rename = "sync-app-commands",
    aliases("sync"),
    owners_only,
    hide_in_help
)]
pub async fn sync_app_commands(ctx: Context<'_>) -> MaybeError {
    poise_register_application_commands_buttons(ctx).await?;
    Ok(())
}
