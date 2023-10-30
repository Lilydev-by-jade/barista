use poise::serenity_prelude::{GuildChannel, Role};

use crate::{
    Error,
    Context
};

#[poise::command(prefix_command, slash_command, subcommands("triage"))]
pub async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn triage(
    ctx: Context<'_>,
    #[description = "Enable triage module"]
    is_enabled: bool,
    #[description = "Channel to send triage request"]
    moderator_channel: Option<GuildChannel>,
    #[description = "Role to give to members if request is accepted"]
    member_role: Option<Role>
) -> Result<(), Error> {
    if is_enabled {
        if moderator_channel.is_none() && member_role.is_none() {
            ctx.send(|m| {
                m.embed(|e| {
                    e
                        .color(0xc92020)
                        .title("Failed to enable config: `triage`")
                        .description("`moderator_channel` and `member_role` must be selected")
                }).ephemeral(true)
            }).await?;
        } else if moderator_channel.is_none() {
            ctx.send(|m| {
                m.embed(|e| {
                    e
                        .color(0xc92020)
                        .title("Failed to enable config: `triage`")
                        .description("`moderator_channel` must be selected")
                }).ephemeral(true)
            }).await?;
        } else if member_role.is_none() {
            ctx.send(|m| {
                m.embed(|e| {
                    e
                        .color(0xc92020)
                        .title("Failed to enable config: `triage`")
                        .description("`member_role` must be selected")
                }).ephemeral(true)
            }).await?;
        } else {
            let guild_id = *ctx.guild().unwrap().id.as_u64() as i64;

            match sqlx::query!(
                r#"
                UPDATE "triage"
                SET enabled = true,
                    moderator_channel_id = $1,
                    member_role_id = $2
                WHERE guild_id = $3
                "#,
                *moderator_channel.unwrap().id.as_u64() as i64,
                *member_role.unwrap().id.as_u64() as i64,
                guild_id
            ).execute(&ctx.data().db).await {
                Ok(_) => {
                    ctx.send(|m| {
                        m.embed(|e| {
                            e
                                .color(0x34eb46)
                                .title("Successfully enabled config: `triage`")
                        }).ephemeral(true)
                    }).await?;
                },
                Err(err) => {
                    ctx.send(|m| {
                        m.embed(|e| {
                            e
                                .color(0xc92020)
                                .title("Failed to enable config: `triage`")
                                .description(err.to_string())
                        }).ephemeral(true)
                    }).await?;
                }
            }
        }
    } else {
        let guild_id = *ctx.guild().unwrap().id.as_u64() as i64;

        match sqlx::query!(
                r#"
                UPDATE "triage"
                SET enabled = false
                WHERE guild_id = $1
                "#, guild_id
            ).execute(&ctx.data().db).await {
            Ok(_) => {
                ctx.send(|m| {
                    m.embed(|e| {
                        e
                            .color(0x34eb46)
                            .title("Successfully disabled config: `triage`")
                    }).ephemeral(true)
                }).await?;
            },
            Err(err) => {
                ctx.send(|m| {
                    m.embed(|e| {
                        e
                            .color(0xc92020)
                            .title("Failed to disable config: `triage`")
                            .description(err.to_string())
                    }).ephemeral(true)
                }).await?;
            }
        }
    }
    Ok(())
}


