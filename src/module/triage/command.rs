use poise::serenity_prelude as serenity;
use serenity::{GuildChannel, Role};

use crate::{common::Context, error::CommandError};

#[poise::command(slash_command, prefix_command)]
pub async fn enable(
    ctx: Context<'_>,
    #[description = "The channel to send triage messages to"] mod_channel: GuildChannel,
    #[description = "The role to give approved users"] access_role: Role,
    #[description = "The role to remove for approved users"] remove_role: Option<Role>,
) -> Result<(), CommandError> {
    let guild_id = i64::from(ctx.guild_id().unwrap());

    let remove_role_id: Option<i64> = remove_role.map(|r| i64::from(r.id));

    sqlx::query!(
        r#"
        UPDATE triage
        SET enabled = true, mod_channel_id = $1, access_role_id = $2, remove_role_id = $3
        WHERE guild_id = $4
        "#,
        i64::from(mod_channel.id),
        i64::from(access_role.id),
        remove_role_id,
        guild_id
    )
    .execute(&ctx.data().db)
    .await?;

    ctx.reply("☑️ Enabled triage module!").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn disable(ctx: Context<'_>) -> Result<(), CommandError> {
    let guild_id = i64::from(ctx.guild_id().unwrap());

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

    Ok(())
}
