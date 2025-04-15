pub mod core;
pub mod models;
pub mod routes;

use actix_web::{App, HttpServer};
use actix_governor::GovernorConfigBuilder;
use actix_governor::Governor;
use log::info;
use crate::core::auth::Auth;
use crate::core::config::load_rustls_config;
use crate::core::database::init_db;
use crate::routes::configure_routes;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Starting API server...");

    let db_pool = init_db().await.map_err(|e| {
        log::error!("Failed to initialize database: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("Database initialization failed: {}", e))
    })?;

    let tls_config = load_rustls_config("certs/cert.pem", "certs/key.pem")
        .expect("Failed to load TLS configuration");

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
        .finish()
        .unwrap();

    log::info!("Server running at https://127.0.0.1:8443");
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