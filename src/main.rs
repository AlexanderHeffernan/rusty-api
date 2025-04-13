use actix_web::{App, HttpServer};
use log::info;
use crate::config::load_rustls_config;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting API server...");

    // Load TLS configuration
    let tls_config = load_rustls_config("certs/cert.pem", "certs/key.pem")
        .expect("Failed to load TLS configuration");

    // Start server
    info!("Server running at https://127.0.0.1:8443");
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
    })
    .bind_rustls_0_23(("127.0.0.1", 8443), tls_config)?
    .run()
    .await
}