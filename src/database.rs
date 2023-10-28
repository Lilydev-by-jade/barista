use std::env;
use sqlx::{
    pool::Pool,
    postgres::PgPool,
    Postgres, Error
};

pub async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let connection_url = env::var("DATABASE_URL").unwrap();
    let connection = PgPool::connect(&*connection_url).await;

    info!("Successfully connected to database!");

    connection
}
