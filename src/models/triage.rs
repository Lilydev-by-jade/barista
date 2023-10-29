use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct TriageDb {
    pub guild_id: u64,
    pub enabled: bool,
    #[serde(rename = "moderator_channel_id")]
    pub moderator_channel: Option<u64>,
    #[serde(rename = "member_role_id")]
    pub member_role: Option<u64>
}
