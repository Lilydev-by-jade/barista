use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, RoleId};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriageConfig {
    pub guild_id: GuildId,
    pub enabled: bool,
    pub mod_channel_id: Option<ChannelId>,
    pub access_role_id: Option<RoleId>,
}
