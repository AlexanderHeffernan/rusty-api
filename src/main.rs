/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This is the main entry point for the Rust API server.
    - Users can modify this file to add middleware or configure the server.
    - For adding new routes, use the `routes/` directory.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

use actix_web::{App, HttpServer};
use actix_governor::GovernorConfigBuilder;
use actix_governor::Governor;
use log::info;
use crate::core::auth::Auth;
use crate::core::config::load_rustls_config;
use crate::core::database::init_db;
use crate::routes::configure_routes;

mod core;
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