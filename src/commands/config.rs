// use poise::command;

// use crate::types::{
//    Context,
//    MaybeError,
//};

///// Returns the latency between the bot and Discord.
//#[command(
//    prefix_command,
//    slash_command,
//    track_edits,
//    broadcast_typing,
//    subcommands("prefix")
//)]
// pub async fn config(ctx: Context<'_>) -> MaybeError {
//    Ok(())
//}

///// Sets the guild specific prefix.
//#[command(
//    prefix_command,
//    slash_command,
//    track_edits,
//    broadcast_typing,
//    guild_only,
//    required_permissions = "MANAGE_GUILD"
//)]
// pub async fn prefix(
//    ctx: Context<'_>,
//    #[rename = "newprefix"]
//    #[description = "The prefix that will apply to the current guild. Leave
// empty to reset to the \                     default prefix."]
//    new_prefix: Option<String>,
//) -> MaybeError {
//    Ok(())
//}
