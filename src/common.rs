use sqlx::PgPool;

pub type Context<'a> = poise::Context<'a, Data, crate::error::CommandError>;
#[derive(Debug, Clone)]
pub struct Data {
    pub db: PgPool,
}
