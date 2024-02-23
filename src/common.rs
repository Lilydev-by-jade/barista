use sqlx::PgPool;

pub type Context<'a> = poise::Context<'a, Data, crate::error::CommandError>;
pub struct Data {
    pub db: PgPool,
}
