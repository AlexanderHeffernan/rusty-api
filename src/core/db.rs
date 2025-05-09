use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:dinner_sync.db".to_string());
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}