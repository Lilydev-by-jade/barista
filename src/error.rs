use thiserror::Error;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("database connection failed")]
    DatabaseConnection(#[from] sqlx::Error),
    #[error("database migration failed")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),
    #[error("bot failed something idk")]
    Bot(#[from] Error),
    #[error("unknown error occurred")]
    Unknown(#[from] anyhow::Error)
}

