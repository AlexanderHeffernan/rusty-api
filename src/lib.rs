/*!
 * [![github]](https://github.com/AlexanderHeffernan/rusty-api)&ensp;[![crates-io]](https://crates.io/crates/rusty-api)&ensp;[![docs-rs]](crate)
 * [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
 * [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
 * [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
 * 
 * Rusty API is lightweight and secure library for building backend APIs in Rust, designed to simplify the development of modern web services.
 * 
 * Features:
 * - **Modular Design**: Oragnized into modules for API logic, routing, and core configurations, making it easy to extend and maintain.
 * - **Actix Web Integration**: Built on the Actix Web framework for high-performance and asynchronous web applications.
 * - **TLS Support**: Includes utilities for configuring Rustls for secure HTTPS communication.
 * - **CORS Handling**: Provides seamless integration with Actix CROS for managing croos-origin requests.
 * - **Password Protection**: Offers built-in support for password-protected routes, enhancing security for sensitive endpoints.
 *
 * Rust API is ideal for developers looking to quickly build secure and scalable APIs with minimal boilerplate.
 *
 * <br>
 * 
 * ## Installation
 * To use rusty-api in your project, add the following to your `Cargo.toml`:
 * ```toml
 * [dependencies]
 * rusty-api = "0.1.8"
 * ```
 * 
 * ## Example
 * ### Setting up your API
 * Here's an example of how to use rusty-api to create an API with public and password-protected routes:
 * ```rust
 * use rusty_api;
 *
 * async fn password_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
 *     rusty_api::HttpResponse::Ok().body("Password route accessed!")
 * }
 *
 * async fn open_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
 *     rusty_api::HttpResponse::Ok().body("Open route accessed!")
 * }
 *
 * fn main() {
 *     let routes = rusty_api::Routes::new()
 *         .add_route_with_password("/password_route", password_route, "Password123")
 *         .add_route("/open_route", open_route);
 *
 *     rusty_api::Api::new()
 *         .certs("certs/cert.pem", "certs/key.pem")
 *         .rate_limit(3, 20)
 *         .bind("127.0.0.1", 8443)
 *         .configure_routes(routes)
 *         .configure_cors(|| {
 *             rusty_api::Cors::default()
 *                 .allow_any_origin()
 *                 .allow_any_method()
 *                 .allowed_header("ngrok-skip-browser-warning")
 *         })
 *         .start();
 * }
 * ```
 *
 * ### Generating Self-Signed Certificates
 * HTTPS requires TLS certificates. You can generate self-signed certificates using OpenSSL:
 * ```bash
 * mkdir -p certs
 * openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem
 * ```
 *
 * ### Running the API
 * To run the API, use the following command:
 * ```bash
 * cargo run
 * ```
 */

pub mod api;
pub mod routes;
pub mod core;

pub use crate::api::Api;
pub use crate::routes::Routes;
pub use crate::core::config::load_rustls_config;

pub use actix_web::{HttpResponse, HttpRequest};
pub use actix_web::http::StatusCode;
pub use actix_cors::Cors;