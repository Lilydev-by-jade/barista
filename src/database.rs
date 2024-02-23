use std::env;

use log::info;
use sqlx::{PgPool, Pool, Postgres};

use crate::error::RuntimeError;

pub async fn create_pool() -> Result<Pool<Postgres>, RuntimeError> {
    let conn_url = env::var("DATABASE_URL")?;
    let conn = PgPool::connect(&conn_url).await?;

    info!("Connected to database!");

    Ok(conn)
}
