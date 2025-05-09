/*!
 * The `api` module provides the core functionality for setting up and running a secure, high-performance API server with rusty-api.
 *
 * This module integrates with the Actix Web framework and includes features such as:
 * - **TLS Support**: Secure communication using Rustls.
 * - **Rate Limiting**: Configurable request limits to prevent abuse.
 * - **CORS Configuration**: Flexible settings for managing cross-origin requests.
 * - **Custom Routes**: Easily define and configure API routes/endpoints.
 *
 * The `Api` struct serves as the main entry point for configuring and starting the server, offering methods for setting
 * up TLS, binding to an address, configuring routes, and more.
 */
use crate::core::config::load_rustls_config;
use crate::routes::Routes;

use actix_web::{App, HttpServer, web};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_cors::Cors;
use std::sync::Arc;

/**
 * The `Api` struct is the main entry point for configuring and running the API server.
 *
 * This struct provides methods to set up TLS, configure routes, manage CORS settings, 
 * apply rate limiting, and bind the server to a specific address and port. It is designed 
 * to simplify the process of building secure and high-performance APIs using the Actix Web framework.
 *
 * # Features
 * - **TLS Support**: Configure paths to certificate and key files for secure HTTPS communication.
 * - **Rate Limiting**: Set request limits to prevent abuse.
 * - **CORS Configuration**: Customize cross-origin resource sharing settings.
 * - **Custom Routes**: Define and configure API routes using the `Routes` builder.
 *
 * # Example
 * ```rust,no_run
 * use rusty_api::Api;
 *
 * let api = Api::new()
 *      .certs("certs/cert.pem", "certs/key.pem")
 *      .rate_limit(5, 10)
 *      .bind("127.0.0.1", 8443)
 *      .configure_cors(|| {
 *          rusty_api::Cors::default()
 *              .allow_any_origin()
 *              .allow_any_method()
 *      });
 * 
 * api.start();
 * ```
 */
pub struct Api {
    /// Path to the TLS certificate file used for secure HTTPS communication.
    cert_path: String,

    /// Path to the private key used for TLS.
    key_path: String,

    /// Address to bind the API server to (e.g., "127.0.0.1").
    addr: String,
    
    /// Port to bind the API server to (e.g., 8443).
    port: u16,

    /// Rate limiting configuration: `(requests_per_second, burst_size)`.
    rate_limit: (u64, u32),

    /// Optional custom routes configuration, provided as a closure.
    custom_routes: Option<Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>,

    /// Custom CORS configuration, provided as a closure.
    custom_cors: Arc<dyn Fn() -> Cors + Send + Sync>,

    /// Optional enable user database.
    user_db: bool,
}

impl Api {
    /**
     * Create a new instance of the API server with default settings.
     *
     * # Returns
     * A new `Api` instance with default values.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new();
     * assert_eq!(api.get_cert_path(), "certs/cert.pem");
     * assert_eq!(api.get_key_path(), "certs/key.pem");
     * assert_eq!(api.get_addr(), "127.0.0.1");
     * assert_eq!(api.get_port(), 8443);
     * assert_eq!(api.get_rate_limit(), (3, 20));
     * ```
     */
    pub fn new() -> Self {
        Self {
            cert_path: "certs/cert.pem".into(),
            key_path: "certs/key.pem".into(),
            addr: "127.0.0.1".into(),
            port: 8443,
            rate_limit: (3, 20),
            custom_routes: None,
            custom_cors: Arc::new(|| Cors::default()),
            user_db: false,
        }
    }

    /**
     * Set the certificate and key paths for TLS.
     *
     * # Arguments
     * * `cert` - Path to the certificate file.
     * * `key` - Path to the private key file.
     *
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().certs("path/to/cert.pem", "path/to/key.pem");
     * assert_eq!(api.get_cert_path(), "path/to/cert.pem");
     * assert_eq!(api.get_key_path(), "path/to/key.pem");
     * ```
     */
    pub fn certs(mut self, cert: &str, key: &str) -> Self {
        self.cert_path = cert.into();
        self.key_path = key.into();
        self
    }

    /**
     * Set the rate limit for API requests.
     *
     * # Arguments
     * * `per_second` - Number of requests allowed per second.
     * * `burst_size` - Maximum burst size for requests.
     *
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```
     * use rusty_api::Api;
     *
     * let api = Api::new().rate_limit(5, 10);
     * assert_eq!(api.get_rate_limit_per_second(), 5);
     * assert_eq!(api.get_rate_limit_burst_size(), 10);
     * ```
     */
    pub fn rate_limit(mut self, per_second: u64, burst_size: u32) -> Self {
        self.rate_limit = (per_second, burst_size);
        self
    }

    /**
     * Set the address and port for the API server.
     *
     * # Arguments
     * * `addr` - Address to bind the server to.
     * * `port` - Port to bind the server to.
     *
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().bind("127.0.0.1", 8443);
     * assert_eq!(api.get_bind_addr(), "127.0.0.1:8443");
     * ```
     */
    pub fn bind(mut self, addr: &str, port: u16) -> Self {
        self.addr = addr.into();
        self.port = port;
        self
    }

    /**
     * Configure custom routes for the API server.
     *
     * # Arguments
     * * `routes` - A `Routes` instance containing the custom routes.
     * 
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     * use rusty_api::Routes;
     *
     * async fn example_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
     *   rusty_api::HttpResponse::Ok().body("Example route accessed!")
     * }
     *
     * let routes = Routes::new()
     *    .add_route("/example", example_route);
     *
     * let api = Api::new()
     *    .configure_routes(routes);
     *
     * assert!(api.get_custom_routes().is_some());
     * ```
     */
    pub fn configure_routes(mut self, routes: Routes) -> Self {
        self.custom_routes = Some(Arc::new(move |cfg| routes.configure(cfg)));
        self
    }

    /**
     * Configure CORS settings.
     *
     * # Arguments
     * * `cors` - A closure that takes a `Cors` instance and returns a modified `Cors` instance.
     *
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```rust
     * use rusty_api;
     *
     * let api = rusty_api::Api::new()
     *     .configure_cors(|| {
     *         rusty_api::Cors::default()
     *             .allow_any_origin()
     *             .allow_any_method()
     *     });
     * ```
     */
    pub fn configure_cors<F>(mut self, cors_config: F) -> Self
    where
        F: Fn() -> Cors + Send + Sync + 'static,
    {
        self.custom_cors = Arc::new(cors_config);
        self
    }

    /**
     * Enable user database.
     *
     * # Returns
     * A mutable reference to the `Api` instance.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().enable_user_db();
     * ```
     */
    pub fn enable_user_db(mut self) -> Self {
        self.user_db = true;
        self
    }

    /**
     * Start the API server.
     * 
     * This method initializes the server and begins listening for incoming requests.
     * It will block the current thread until the server is stopped.
     *
     * # Example
     * ```rust,no_run
     * use rusty_api::Api;
     *
     * let api = Api::new().start();
     */
    pub fn start(self) {
        let rt = actix_web::rt::System::new();
        if let Err(e) = rt.block_on(async {
            println!("INFO: Starting API server...");

            dotenv::dotenv().ok();
            let pool = if self.user_db {
                Some(crate::core::db::init_db().await.expect("Failed to init DB"))
            } else {
                None
            };

            let tls_config = load_rustls_config(&self.cert_path, &self.key_path).expect("TLS failed");

            let governor_config = GovernorConfigBuilder::default()
                .per_second(self.rate_limit.0)
                .burst_size(self.rate_limit.1)
                .finish()
                .unwrap();

            let cors_config = self.custom_cors.clone();

            let bind_addr = format!("{}:{}", self.addr, self.port);

            println!("INFO: Server binding to {}", bind_addr);
            HttpServer::new(move || {
            let cors = (cors_config)();
                let mut app = App::new()
                    .wrap(cors)
                    .wrap(Governor::new(&governor_config));

                // Add app_data for the pool if it exists
                if let Some(pool) = pool.clone() {
                    app = app.app_data(web::Data::new(pool));
                    // Configure routes::configure_routes
                    app = app.configure(crate::core::routes::configure_routes);
                }

                // Apply custom routes if provided
                if let Some(custom_routes) = &self.custom_routes {
                    app = app.configure(|cfg| custom_routes(cfg));
                }

                app
            })
            .bind_rustls_0_23((self.addr.to_string(), self.port), tls_config)?
            .run()
            .await
        }) {
            println!("ERROR: Failed to start API server: {:?}", e);
        }
    }

    /**
     * Get the path to the TLS certificate file.
     *
     * # Returns
     * A string representing the path to the certificate file.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().certs("path/to/cert.pem", "path/to/key.pem");
     * assert_eq!(api.get_cert_path(), "path/to/cert.pem");
     * ```
     */
    pub fn get_cert_path(&self) -> &str { &self.cert_path }

    /**
     * Get the path to the TLS private key file.
     *
     * # Returns
     * A string representing the path to the private key file.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().certs("path/to/cert.pem", "path/to/key.pem");
     * assert_eq!(api.get_key_path(), "path/to/key.pem");
     * ```
     */
    pub fn get_key_path(&self) -> &str { &self.key_path }

    /**
     * Get the address the server is bound to.
     *
     * # Returns
     * A string representing the address.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     * 
     * let api = Api::new().bind("123.4.5.6", 7891);
     * assert_eq!(api.get_addr(), "123.4.5.6");
     * ```
     */
    pub fn get_addr(&self) -> &str { &self.addr }

    /**
     * Get the port the server is bound to.
     *
     * # Returns
     * A u16 representing the port.
     * 
     * # Example
     * ```rust
     * use rusty_api::Api;
     * 
     * let api = Api::new().bind("123.4.5.6", 7891);
     * assert_eq!(api.get_port(), 7891);
     * ```
     */
    pub fn get_port(&self) -> u16 { self.port }

    /**
     * Get the rate limit configuration.
     *
     * # Returns
     * A tuple containing the rate limit configuration: `(requests_per_second, burst_size)`.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().rate_limit(5, 10);
     * assert_eq!(api.get_rate_limit(), (5, 10));
     * ```
     */
    pub fn get_rate_limit(&self) -> (u64, u32) { self.rate_limit }

    /**
     * Get the address and port the server is bound to as a single string.
     *
     * # Returns
     * A string representing the address and port in the format "address:port".
     * 
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().bind("123.4.5.6", 7891);
     * assert_eq!(api.get_bind_addr(), "123.4.5.6:7891");
     * ```
     */
    pub fn get_bind_addr(&self) -> String { format!("{}:{}", self.addr, self.port) }

    /**
     * Get the rate limit per second.
     *
     * # Returns
     * A u64 representing the number of requests allowed per second.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().rate_limit(5, 10);
     * assert_eq!(api.get_rate_limit_per_second(), 5);
     * ```
     */
    pub fn get_rate_limit_per_second(&self) -> u64 { self.rate_limit.0 }

    /**
     * Get the rate limit burst size.
     *
     * # Returns
     * A u32 representing the maximum burst size for requests.
     *
     * # Example
     * ```rust
     * use rusty_api::Api;
     *
     * let api = Api::new().rate_limit(5, 10);
     * assert_eq!(api.get_rate_limit_burst_size(), 10);
     * ```
     */
    pub fn get_rate_limit_burst_size(&self) -> u32 { self.rate_limit.1 }

    /**
     * Get the custom CORS configuration.
     *
     * # Returns
     * A reference to the custom CORS configuration closure.
     */
    pub fn get_custom_routes(&self) -> Option<&Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>> { self.custom_routes.as_ref() }
}