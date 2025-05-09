use crate::core::user::{LoginResponse, User};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{ok, Ready};
use std::env;
use sqlx::Row;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, 12)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn generate_jwt(user: &User) -> String {
    let claims = Claims {
        sub: user.id,
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub async fn register_user(
    pool: &sqlx::SqlitePool,
    input: crate::core::user::RegisterInput,
) -> Result<User, String> {
    // Hash password
    let password_hash = hash_password(&input.password).map_err(|e| e.to_string())?;
    
    // Insert user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id, username, password_hash"
    )
    .bind(&input.username)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(user)
}

pub async fn login_user(
    pool: &sqlx::SqlitePool,
    input: crate::core::user::LoginInput,
) -> Result<LoginResponse, String> {
    // Find user
    let row = sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ?")
        .bind(&input.username)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;

    let user = User {
        id: row.get("id"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
    };
    
    // Verify password
    if !verify_password(&input.password, &user.password_hash) {
        return Err("Invalid password".to_string());
    }
    
    // Generate JWT
    let token = generate_jwt(&user);
    Ok(LoginResponse { token })
}

/**
 * Middleware to extract and validate JWT token from the request.
 */
pub fn validate_token(token: &str) -> Result<Claims, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    ) {
        Ok(decoded) => Ok(decoded.claims),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}