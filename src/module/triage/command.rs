use poise::serenity_prelude as serenity;
use serenity::{GuildChannel, Role};

use crate::{common::Context, error::CommandError, module::triage::TriageError};

#[poise::command(slash_command, prefix_command)]
pub async fn triage(
    ctx: Context<'_>,
    #[description = "Enable or disable the triage module"] enabled: bool,
    #[description = "The channel to send triage messages to"] mod_channel: Option<GuildChannel>,
    #[description = "The role to give approved users"] access_role: Option<Role>,
) -> Result<(), CommandError> {
    let guild_id = i64::from(ctx.guild_id().unwrap());

    if enabled {
        if mod_channel.is_none() {
            return Err(CommandError::TriageError(TriageError::ChannelNotSelected));
        }
        if access_role.is_none() {
            return Err(CommandError::TriageError(TriageError::RoleNotSelected));
        }

        sqlx::query!(
            r#"
            UPDATE triage
            SET enabled = true, mod_channel_id = $1, access_role_id = $2
            WHERE guild_id = $3
            "#,
            i64::from(mod_channel.unwrap().id),
            i64::from(access_role.unwrap().id),
            guild_id
        )
        .execute(&ctx.data().db)
        .await?;

        ctx.reply("☑️ Enabled triage module!").await?;
    } else {
        sqlx::query!(
            r#"
            UPDATE triage
            SET enabled = false
            WHERE guild_id = $1
            "#,
            guild_id
        )
        .execute(&ctx.data().db)
        .await?;

        ctx.reply("☑️ Disabled triage module!").await?;
    }
    Ok(())
}
