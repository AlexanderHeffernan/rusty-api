pub mod api;
pub mod routes;
pub mod config;

pub use api::Api;
pub use routes::Routes;
pub use config::load_rustls_config;
pub use actix_web::{HttpResponse, HttpRequest};
pub use actix_web::http::StatusCode; // Corrected import
pub use actix_cors::Cors;

pub fn check_password(req: &HttpRequest, expected_password: &str) -> bool {
    // Extract the query string from the request
    let query_string = req.query_string();

    // Parse the query string into key-value pairs
    if let Ok(params) = serde_urlencoded::from_str::<std::collections::HashMap<String, String>>(query_string) {
        // Check if the "password" parameter matches the expected password
        if let Some(password) = params.get("password") {
            return password == expected_password;
        }
    }

    false
}