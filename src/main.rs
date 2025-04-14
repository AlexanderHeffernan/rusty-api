use actix_web::{App, HttpServer};
use actix_governor::GovernorConfigBuilder;
use actix_governor::Governor;
use log::info;
use crate::auth::Auth;
use crate::config::load_rustls_config;
use crate::database::init_db;
use crate::routes::configure_routes;

mod auth;
mod config;
mod database;
mod models;
mod routes;

/*
    This is a Rust API server template, built for secure and efficient web applications.
    Supports HTTPS, authentication, and rate limiting.
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting API server...");

    // Initialize the database for authentication and user management
    let db_pool = init_db().await.map_err(|e| {
        log::error!("Failed to initialize database: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("Database initialization failed: {}", e))
    })?;

    // Load TLS configuration for HTTPS
    let tls_config = load_rustls_config("certs/cert.pem", "certs/key.pem")
        .expect("Failed to load TLS configuration");

    // Configure rate limiting using actix-governor
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(3) // 3 requests per second
        .burst_size(20) // Allow burst of 20 requests
        .finish()
        .unwrap();

    info!("Server running at https://127.0.0.1:8443");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .wrap(Governor::new(&governor_conf))
            .wrap(Auth::new(db_pool.clone()))
            .configure(configure_routes)
    })
    .bind_rustls_0_23(("127.0.0.1", 8443), tls_config)?
    .run()
    .await
}