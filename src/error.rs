#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(#[from] sqlx::Error),
    #[error("Environment variable error: {0}")]
    EnvVarNotFound(#[from] std::env::VarError),
    #[error("Migrations failed: {0}")]
    MigrationsFailed(#[from] sqlx::migrate::MigrateError),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Message error: {0}")]
    MessageNotSent(#[from] poise::serenity_prelude::Error),
    #[error("Event error: {0}")]
    Event(#[from] EventError),
}

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("SQL error: {0}")]
    SqlError(#[from] sqlx::Error),
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
    #[error("Triage error: {0}")]
    TriageError(#[from] crate::module::triage::TriageError),
}
