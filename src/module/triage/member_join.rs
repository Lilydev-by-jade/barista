use log::info;
use poise::serenity_prelude::{
    self as serenity, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage,
};
use serenity::{ChannelId, RoleId};

use crate::{error::EventError, models::EventInfo, module::triage::models::TriageConfig};

use super::TriageError;

pub async fn handle_triage_request(
    event_info: EventInfo<'_>,
    new_member: &serenity::model::guild::Member,
) -> Result<(), EventError> {
    let triage_conf = sqlx::query_as!(
        TriageConfig,
        r#"
        SELECT * FROM "triage" WHERE "guild_id" = $1
        "#,
        i64::from(new_member.guild_id)
    )
    .fetch_one(&event_info.data.db)
    .await?;

    if triage_conf.enabled {
        let approve_button_id = format!("approve_{}", new_member.user.id);
        let kick_button_id = format!("kick_{}", new_member.user.id);
        let ban_button_id = format!("ban_{}", new_member.user.id);

        let mod_channel = match triage_conf.mod_channel_id {
            Some(id) => {
                let ctx = &event_info.ctx;
                ctx.http.get_channel(ChannelId::new(id as u64)).await?
            }
            None => {
                return Err(EventError::TriageError(TriageError::ChannelNotFound));
            }
        };
        let _access_role = match triage_conf.access_role_id {
            Some(id) => match RoleId::new(id as u64).to_role_cached(&event_info.ctx.cache) {
                Some(role) => role,
                None => {
                    return Err(EventError::TriageError(TriageError::RoleNotFound));
                }
            },
            None => {
                return Err(EventError::TriageError(TriageError::RoleNotFound));
            }
        };

        mod_channel
            .id()
            .send_message(
                &event_info.ctx,
                CreateMessage::new()
                    .embed(
                        CreateEmbed::new()
                            .title("User awaiting approval")
                            .description(format!(
                                "User: <@{}> `{}#{:?}`\nID: {}",
                                new_member.user.id,
                                new_member.user.name,
                                new_member.user.discriminator,
                                new_member.user.id
                            ))
                            .color(0x00ff00)
                            .footer(CreateEmbedFooter::new(format!(
                                "User joined at: {}",
                                new_member.user.created_at().format("%b, %e %Y %r"),
                            ))),
                    )
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new(approve_button_id.to_string())
                            .label("Approve")
                            .style(serenity::ButtonStyle::Primary),
                        CreateButton::new(kick_button_id.to_string())
                            .label("Kick")
                            .style(serenity::ButtonStyle::Danger),
                        CreateButton::new(ban_button_id.to_string())
                            .label("Ban")
                            .style(serenity::ButtonStyle::Danger),
                    ])]),
            )
            .await?;
    } else {
        info!(
            "Triage is enabled for guild with ID: {}",
            new_member.guild_id
        );
    }

    Ok(())
}
