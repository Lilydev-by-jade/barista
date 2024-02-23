use log::info;
use poise::serenity_prelude as serenity;
use serenity::model::guild::Guild;

use crate::{error::EventError, models::EventInfo};

pub async fn handle_guild_create(
    event_info: EventInfo<'_>,
    guild: &Guild,
    is_new: &Option<bool>,
) -> Result<(), EventError> {
    match is_new {
        Some(is_new) => match is_new {
            true => {
                info!("Joined guild with ID: {}", guild.id);
            }
            false => {
                info!("Rejoined guild with ID: {}", guild.id);
            }
        },
        None => {
            info!("Joined guild with ID: {}", guild.id);
        }
    }

    let guild_entry_exists = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM "triage" WHERE "guild_id" = $1)
        "#,
        i64::from(guild.id)
    )
    .fetch_one(&event_info.data.db)
    .await?
    .exists
    .unwrap_or(false);

    if !guild_entry_exists {
        sqlx::query!(
            r#"
            INSERT INTO "triage" ("guild_id", "enabled") VALUES ($1, $2)
            "#,
            i64::from(guild.id),
            false
        )
        .execute(&event_info.data.db)
        .await?;
    }

    Ok(())
}
