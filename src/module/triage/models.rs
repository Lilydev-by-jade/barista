use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriageConfig {
    pub guild_id: i64,
    pub enabled: bool,
    pub mod_channel_id: Option<i64>,
    pub access_role_id: Option<i64>,
    pub remove_role_id: Option<i64>,
}
