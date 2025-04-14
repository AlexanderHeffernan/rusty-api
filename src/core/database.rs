/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This module handles the database connection and initialization.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

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

    info!("Database connection established.");
    Ok(pool)
}