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
    #[error("User does not have role {0}")]
    UserRoleNotFound(String),
    #[error("{0}")]
    ChannelNotSelected(String),
    #[error("{0}")]
    RoleNotSelected(String),
    #[error("{0}")]
    ChannelAndRoleNotSelected(String),
    #[error("Could not parse user ID: {0}")]
    UserIdParse(#[from] std::num::ParseIntError),
}
