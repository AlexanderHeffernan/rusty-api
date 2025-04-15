use actix_web::{App, HttpServer};
use actix_governor::GovernorConfigBuilder;
use actix_governor::Governor;
use log::info;
use crate::core::auth::Auth;
use crate::core::config::load_rustls_config;
use crate::core::database::init_db;
use crate::routes::configure_routes;

pub struct Api {
    cert_path: String,
    key_path: String,
    db_path: String,
    addr: String,
    port: u16,
    rate_limit: (u64, u32),
}

impl Api {
    /// Create a new instance of the API server with default settings.
    ///
    /// # Returns
    /// A new `Api` instance with default values.
    ///
    /// # Example
    /// ```
    /// use rusty_api::Api;
    ///
    /// let api = Api::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cert_path: "certs/cert.pem".into(),
            key_path: "certs/key.pem".into(),
            db_path: "users.db".into(),
            addr: "127.0.0.1".into(),
            port: 8443,
            rate_limit: (3, 20),
        }
    }

    /// Set the certificate and key paths for TLS.
    ///
    /// # Arguments
    /// * `cert` - Path to the certificate file.
    /// * `key` - Path to the private key file.
    ///
    /// # Returns
    /// A mutable reference to the `Api` instance.
    ///
    /// # Example
    /// ```
    /// use rusty_api::Api;
    ///
    /// let api = Api::new().certs("path/to/cert.pem", "path/to/key.pem");
    /// assert_eq!(api.get_cert_path(), "path/to/cert.pem");
    /// assert_eq!(api.get_key_path(), "path/to/key.pem");
    /// ```
    pub fn certs(mut self, cert: &str, key: &str) -> Self {
        self.cert_path = cert.into();
        self.key_path = key.into();
        self
    }

    /// Set the path to the SQLite database file.
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file.
    ///
    /// # Returns
    /// A mutable reference to the `Api` instance.
    ///
    /// # Example
    /// ```
    /// use rusty_api::Api;
    ///
    /// let api = Api::new().auth_db("path/to/users.db");
    /// assert_eq!(api.get_db_path(), "path/to/users.db");
    /// ```
    pub fn auth_db(mut self, path: &str) -> Self {
        self.db_path = path.into();
        self
    }

    /// Set the rate limit for API requests.
    ///
    /// # Arguments
    /// * `per_second` - Number of requests allowed per second.
    /// * `burst_size` - Maximum burst size for requests.
    ///
    /// # Returns
    /// A mutable reference to the `Api` instance.
    ///
    /// # Example
    /// ```
    /// use rusty_api::Api;
    ///
    /// let api = Api::new().rate_limit(5, 10);
    /// assert_eq!(api.get_rate_limit_per_second(), 5);
    /// assert_eq!(api.get_rate_limit_burst_size(), 10);
    /// ```
    pub fn rate_limit(mut self, per_second: u64, burst_size: u32) -> Self {
        self.rate_limit = (per_second, burst_size);
        self
    }

    /// Set the address and port for the API server.
    ///
    /// # Arguments
    /// * `addr` - Address to bind the server to.
    /// * `port` - Port to bind the server to.
    ///
    /// # Returns
    /// A mutable reference to the `Api` instance.
    ///
    /// # Example
    /// ```
    /// use rusty_api::Api;
    ///
    /// let api = Api::new().bind("127.0.0.1", 8443);
    /// assert_eq!(api.get_bind_addr(), "127.0.0.1:8443");
    /// ```
    pub fn bind(mut self, addr: &str, port: u16) -> Self {
        self.addr = addr.into();
        self.port = port;
        self
    }

    /// Start the API server.
    ///
    /// # Example:
    /// ```
    /// use rusty_api::Api;
    /// let api = Api::new()
    ///     .certs("path/to/cert.pem", "path/to/key.pem")
    ///     .auth_db("path/to/users.db")
    ///     .rate_limit(5, 10)
    ///     .bind("127.0.0.1", 8443)
    ///     .start();
    /// ```
    pub fn start(self) {
        let rt = actix_web::rt::System::new();
        rt.block_on(async {
            env_logger::init();
            log::info!("Starting API server...");

            let db_pool = init_db().await.expect("DB failed");
            let tls_config = load_rustls_config(&self.cert_path, &self.key_path).expect("TLS failed");

            let governor_conf = GovernorConfigBuilder::default()
                .per_second(self.rate_limit.0)
                .burst_size(self.rate_limit.1)
                .finish()
                .unwrap();

            let bind_addr = format!("{}:{}", self.addr, self.port);

            log::info!("Server running at https://{}", bind_addr);
            HttpServer::new(move || {
                App::new()
                    .app_data(actix_web::web::Data::new(db_pool.clone()))
                    .wrap(Governor::new(&governor_conf))
                    .wrap(Auth::new(db_pool.clone()))
                    .configure(configure_routes)
            })
            .bind_rustls_0_23((self.addr.to_string(), self.port), tls_config)?
            .run()
            .await
        });
    }

    pub fn get_cert_path(&self) -> &str { &self.cert_path }
    pub fn get_key_path(&self) -> &str { &self.key_path }
    pub fn get_db_path(&self) -> &str { &self.db_path }
    pub fn get_addr(&self) -> &str { &self.addr }
    pub fn get_port(&self) -> u16 { self.port }
    pub fn get_rate_limit(&self) -> (u64, u32) { self.rate_limit }
    pub fn get_bind_addr(&self) -> String { format!("{}:{}", self.addr, self.port) }
    pub fn get_rate_limit_per_second(&self) -> u64 { self.rate_limit.0 }
    pub fn get_rate_limit_burst_size(&self) -> u32 { self.rate_limit.1 }
}