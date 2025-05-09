/*!
 * The auth_routes module for handling user authentication and registration.
 *
 * This module defines the routes for user login and registration, including
 * the necessary input and output structures. It uses Actix Web for routing
 * and SQLx for database interaction.
 */
use actix_web::{web, HttpResponse, HttpMessage};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::core::auth::{login_user, register_user, validate_token, Claims};
use crate::core::user::{LoginInput, RegisterInput};

/**
 * Configure routes for user authentication and registration.
 *
 * This function sets up the routes for user login and registration, using
 * Actix Web's `ServiceConfig`. It also applies middleware for JWT validation.
 *
 * # Arguments
 * - `cfg`: A mutable reference to the Actix Web `ServiceConfig`.
 */
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig, login_path: &str, register_path: &str) {
    cfg.route(login_path, web::post().to(login))
       .route(register_path, web::post().to(register));
}

/**
 * Login route handler.
 * 
 * This function handles user login requests. It extracts the login input
 * from the request, calls the `login_user` function to authenticate the user,
 * and returns a JSON response with the login token or an error message.
 *
 * # Arguments
 * - `pool`: A reference to the SQLx SQLite connection pool.
 * - `input`: The login input data, containing the username and password.
 *
 * # Returns
 * An `HttpResponse` containing the login token or an error message.
 */
async fn login(
    pool: web::Data<sqlx::SqlitePool>,
    input: web::Json<LoginInput>,
) -> HttpResponse {
    match login_user(&pool, input.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

/**
 * Register route handler.
 * 
 * This function handles user registration requests. It extracts the registration
 * input from the request, calls the `register_user` function to create a new user,
 * and returns a JSON response with the user data or an error message.
 *
 * # Arguments
 * - `pool`: A reference to the SQLx SQLite connection pool.
 * - `input`: The registration input data, containing the username and password.
 *
 * # Returns
 * An `HttpResponse` containing the user data or an error message.
 */
async fn register(
    pool: web::Data<sqlx::SqlitePool>,
    input: web::Json<RegisterInput>,
) -> HttpResponse {
    match register_user(&pool, input.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}