use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use actix_web::HttpResponse;
use crate::DB_POOL;

pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:dinner_sync.db".to_string());
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}

pub async fn get_user_field(user_id: i32, field: &str) -> HttpResponse {
    let query = format!("SELECT {} FROM users WHERE id = ?", field);
    let result: Option<(String,)> = match sqlx::query_as(&query)
        .bind(user_id)
        .fetch_optional(&*DB_POOL)
        .await
    {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    match result {
        Some((value,)) => HttpResponse::Ok().body(value),
        None => HttpResponse::NotFound().body(format!("Field '{}' not found for user", field)),
    }
}

pub async fn set_user_field(user_id: i32, field: &str, value: &str) -> HttpResponse {
    let query = format!("UPDATE users SET {} = ? WHERE id = ?", field);
    let result = sqlx::query(&query)
        .bind(value)
        .bind(user_id)
        .execute(&*DB_POOL)
        .await;

    match result {
        Ok(rows_affected) if rows_affected.rows_affected() > 0 => {
            HttpResponse::Ok().body(format!("Field '{}' updated successfully", field))
        }
        Ok(_) => HttpResponse::NotFound().body(format!("User with ID '{}' not found", user_id)),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}