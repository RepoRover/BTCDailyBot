use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

lazy_static! {
    pub static ref POOL: OnceCell<Arc<sqlx::PgPool>> = OnceCell::new();
}

pub async fn init_pool() -> Result<Arc<sqlx::PgPool>, sqlx::Error> {
    let database_url: String = env::var("DATABASE_URL").expect("No database url");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(200)
        .connect(database_url.as_str())
        .await?;
    Ok(Arc::new(pool))
}
