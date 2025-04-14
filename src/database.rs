use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use log::info;

/*
    Initiate user database.
    This function creates a SQLite database if it doesn't already exist.
    The database has columns for id, email, api_key, and privilege_level.
    The function also inserts two test users with different privilege levels.
*/
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "sqlite://users.db";
    info!("Connecting to database: {}", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await?;

    info!("Inserting test users...");
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO users (email, api_key, privilege_level)
        VALUES
            ('user@example.com', ?, 1),
            ('admin@example.com', ?, 2)
        "#,
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(uuid::Uuid::new_v4().to_string())
    .execute(&pool)
    .await?;

    Ok(pool)
}