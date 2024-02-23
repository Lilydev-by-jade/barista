use log::info;
use poise::serenity_prelude as serenity;
use serenity::model::guild::Guild;

use crate::{error::EventError, models::EventInfo};

pub async fn handle_guild_create(
    _event_info: EventInfo<'_>,
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
    Ok(())
}
