use actix_web::{App, HttpServer};
use log::info;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::auth::Auth;
use crate::config::load_rustls_config;

mod auth;
mod config;
mod models;
mod routes;

async fn init_db() -> Result<SqlitePool, sqlx::Error> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting API server...");

    let db_pool = match init_db().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database initialization failed: {}", e),
            ));
        }
    };

    let tls_config = load_rustls_config("certs/cert.pem", "certs/key.pem")
        .expect("Failed to load TLS configuration");

    info!("Server running at https://127.0.0.1:8443");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .wrap(Auth::new(db_pool.clone()))
            .service(routes::hello)
            .service(routes::admin)
    })
    .bind_rustls_0_23(("127.0.0.1", 8443), tls_config)?
    .run()
    .await
}