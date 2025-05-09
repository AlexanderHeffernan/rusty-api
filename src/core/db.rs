use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use crate::DB_POOL;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:dinner_sync.db".to_string());
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}

pub async fn get_user_field(user_id: i32, field: &str) -> Result<Option<String>, sqlx::Error> {
    let query = format!("SELECT {} FROM users WHERE id = ?", field);
    let result: Option<(String,)> = sqlx::query_as(&query)
        .bind(user_id)
        .fetch_optional(&*DB_POOL)
        .await?;

    Ok(result.map(|row| row.0))
}