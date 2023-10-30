use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct TriageDb {
    pub guild_id: i64,
    pub enabled: bool,
    pub moderator_channel_id: Option<i64>,
    pub member_role_id: Option<i64>
}
