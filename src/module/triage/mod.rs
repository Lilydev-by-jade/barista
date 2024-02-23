pub mod command;
pub mod guild_create;
pub mod member_join;
pub mod models;

#[derive(Debug, thiserror::Error)]
pub enum TriageError {
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Role not found")]
    RoleNotFound,
    #[error("Channel not selected")]
    ChannelNotSelected,
    #[error("Role not selected")]
    RoleNotSelected,
}
