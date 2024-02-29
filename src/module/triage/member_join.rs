use chrono::Utc;
use log::info;
use poise::serenity_prelude as serenity;
use serenity::{
    ChannelId, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponseMessage, CreateMessage, RoleId, UserId,
};

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

        mod_channel
            .id()
            .send_message(
                &event_info.ctx,
                CreateMessage::new()
                    .content("@here")
                    .embed(
                        CreateEmbed::new()
                            .title("🔔 User awaiting approval!")
                            .description(format!(
                                "**User:** <@{}> `{}`\n**ID:** `{}`\n**Joined at:** <t:{}:f>",
                                new_member.user.id,
                                new_member.user.name,
                                new_member.user.id,
                                new_member.joined_at.unwrap().timestamp()
                            ))
                            .color(0x74c7ec)
                            .footer(
                                CreateEmbedFooter::new(format!(
                                    "User created at: {}",
                                    new_member.user.created_at().format("%b, %e %Y %r"),
                                ))
                                .icon_url(
                                    match new_member.user.avatar_url() {
                                        Some(url) => url,
                                        None => new_member.user.face(),
                                    },
                                ),
                            ),
                    )
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new(approve_button_id.to_string())
                            .label("Approve")
                            .style(serenity::ButtonStyle::Success),
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

pub async fn handle_triage_interaction(
    event_info: EventInfo<'_>,
    interaction: &serenity::model::application::Interaction,
) -> Result<(), EventError> {
    let interaction_custom_id = interaction
        .as_message_component()
        .unwrap()
        .data
        .custom_id
        .clone();
    let guild_id = interaction
        .as_message_component()
        .unwrap()
        .guild_id
        .unwrap();
    let user_id = UserId::new(
        match interaction_custom_id
            .replace("approve_", "")
            .replace("kick_", "")
            .replace("ban_", "")
            .parse::<u64>()
        {
            Ok(id) => id,
            Err(e) => {
                return Err(EventError::TriageError(TriageError::UserIdParse(e)));
            }
        },
    );
    let user = user_id.to_user(&event_info.ctx).await?;

    if interaction_custom_id.starts_with("approve_") {
        let access_role_id = RoleId::new(
            sqlx::query!(
                r#"
                SELECT "access_role_id" FROM "triage" WHERE "guild_id" = $1
                "#,
                i64::from(guild_id)
            )
            .fetch_one(&event_info.data.db)
            .await?
            .access_role_id
            .unwrap() as u64,
        );

        event_info
            .ctx
            .http
            .add_member_role(
                guild_id,
                user_id,
                access_role_id,
                Some("Approved user from triage."),
            )
            .await?;

        if let Some(remove_role_id) = sqlx::query!(
            r#"
            SELECT "remove_role_id"
            FROM "triage"
            WHERE "guild_id" = $1
            "#,
            i64::from(guild_id)
        )
        .fetch_one(&event_info.data.db)
        .await?
        .remove_role_id
        {
            let role = RoleId::new(remove_role_id as u64);

            if user.has_role(&event_info.ctx, guild_id, role).await? {
                event_info
                    .ctx
                    .http
                    .remove_member_role(
                        guild_id,
                        user_id,
                        RoleId::new(remove_role_id as u64),
                        Some("Approved user from triage."),
                    )
                    .await?;
            } else {
                match role.to_role_cached(&event_info.ctx) {
                    Some(role) => {
                        return Err(EventError::TriageError(TriageError::UserRoleNotFound(
                            format!("{} (id: {})", role.name, role.id),
                        )));
                    }
                    None => {
                        return Err(EventError::TriageError(TriageError::RoleNotFound));
                    }
                };
            }
        };

        interaction
            .as_message_component()
            .unwrap()
            .create_response(
                event_info.ctx,
                serenity::CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content("")
                        .embed(
                            CreateEmbed::new()
                                .title("✅ User approved!")
                                .color(0x4BB543)
                                .description(format!(
                                    "**User:** <@{}> `{}`\n**ID:** `{}`",
                                    user.id, user.name, user.id
                                ))
                                .footer(
                                    CreateEmbedFooter::new(format!(
                                        "User approved at: {}",
                                        Utc::now().format("%b, %e %Y %r")
                                    ))
                                    .icon_url(
                                        match user.avatar_url() {
                                            Some(url) => url,
                                            None => user.face(),
                                        },
                                    ),
                                ),
                        )
                        .components(vec![]),
                ),
            )
            .await?;
    } else if interaction_custom_id.starts_with("kick_") {
        event_info
            .ctx
            .http
            .kick_member(guild_id, user_id, Some("User failed triage (kicked)"))
            .await?;

        interaction
            .as_message_component()
            .unwrap()
            .create_response(
                event_info.ctx,
                serenity::CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content("")
                        .embed(
                            CreateEmbed::new()
                                .title("❌ User kicked!")
                                .color(0xdd2e44)
                                .description(format!(
                                    "**User:** <@{}> `{}`\n**ID:** `{}`",
                                    user.id, user.name, user.id
                                ))
                                .footer(
                                    CreateEmbedFooter::new(format!(
                                        "User kicked at: {}",
                                        Utc::now().format("%b, %e %Y %r")
                                    ))
                                    .icon_url(
                                        match user.avatar_url() {
                                            Some(url) => url,
                                            None => user.face(),
                                        },
                                    ),
                                ),
                        )
                        .components(vec![]),
                ),
            )
            .await?;
    } else if interaction_custom_id.starts_with("ban_") {
        event_info
            .ctx
            .http
            .ban_user(guild_id, user_id, 0, Some("User failed triage (banned)"))
            .await?;

        interaction
            .as_message_component()
            .unwrap()
            .create_response(
                event_info.ctx,
                serenity::CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content("")
                        .embed(
                            CreateEmbed::new()
                                .title("❌ User banned!")
                                .color(0xdd2e44)
                                .description(format!(
                                    "**User:** <@{}> `{}`\n**ID:** `{}`",
                                    user.id, user.name, user.id
                                ))
                                .footer(
                                    CreateEmbedFooter::new(format!(
                                        "User banned at: {}",
                                        Utc::now().format("%b, %e %Y %r")
                                    ))
                                    .icon_url(
                                        match user.avatar_url() {
                                            Some(url) => url,
                                            None => user.face(),
                                        },
                                    ),
                                ),
                        )
                        .components(vec![]),
                ),
            )
            .await?;
    }

    Ok(())
}
