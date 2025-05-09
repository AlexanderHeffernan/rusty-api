/*!
 * Database module.
 *
 * This module handles the database connection and provides functions to interact
 * with the database, including querying and updating user fields.
 */
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use actix_web::HttpResponse;
use crate::DB_POOL;

/**
 * Initialize the database connection.
 *
 * This function creates a connection pool to the SQLite database specified in
 * the `DATABASE_URL` environment variable. If the variable is not set, it defaults
 * to `sqlite:./users.db.db`.
 *
 * # Returns
 * A `Result` containing the connection pool or an error if the connection fails.
 */
pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:./users.db".to_string());
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}


/**
 * Get a user field from the database.
 *
 * This function retrieves a specific field from the `users` table for a given user ID.
 *
 * # Arguments
 * - `user_id`: The ID of the user to retrieve the field for.
 * - `field`: The name of the field to retrieve.
 *
 * # Returns
 * An `HttpResponse` containing the value of the field or an error message if the field is not found.
 */
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

/**
 * Set a user field in the database.
 *
 * This function updates a specific field in the `users` table for a given user ID.
 *
 * # Arguments
 * - `user_id`: The ID of the user to update the field for.
 * - `field`: The name of the field to update.
 * - `value`: The new value to set for the field.
 *
 * # Returns
 * An `HttpResponse` indicating the success or failure of the operation.
 */
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