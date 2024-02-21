#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Serenity error: {0}")]
    Serenity(#[from] poise::serenity_prelude::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Message error: {0}")]
    MessageNotSent(#[from] poise::serenity_prelude::Error),
}
