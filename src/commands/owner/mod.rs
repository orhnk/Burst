use crate::types::CommandVec;

mod sync_app_commands;

pub fn commands() -> CommandVec {
    vec![sync_app_commands::sync_app_commands(), prefix()]
}

// TODO: temporary
#[poise::command(
    prefix_command,
    track_edits,
    broadcast_typing,
    category = "Owner",
    guild_only,
    owners_only,
    hide_in_help
)]
pub async fn prefix(
    ctx: crate::types::Context<'_>,
    #[rest] prefix: String,
) -> crate::types::MaybeError {
    if prefix == "reset" {
        sqlx::query("DELETE FROM prefixes WHERE id = ?")
            .bind(ctx.guild_id().unwrap().0 as i64)
            .execute(&ctx.data().db)
            .await?;

        ctx.say("Prefix reset to default.").await?;
    }
    else {
        sqlx::query(
            "INSERT INTO prefixes (id, prefix) VALUES (?, ?) ON CONFLICT (id) DO UPDATE SET \
             prefix = ?",
        )
        .bind(ctx.guild_id().unwrap().0 as i64)
        .bind(prefix.clone())
        .bind(prefix.clone())
        .execute(&ctx.data().db)
        .await?;

        ctx.say(format!("Prefix set to `{}`.", prefix)).await?;
    }

    Ok(())
}
