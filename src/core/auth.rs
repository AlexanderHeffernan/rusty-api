use crate::core::user::{LoginResponse, User};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
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
        "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING *"
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
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&input.username)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("User not found")?;
    
    // Verify password
    if !verify_password(&input.password, &user.password_hash) {
        return Err("Invalid password".to_string());
    }
    
    // Generate JWT
    let token = generate_jwt(&user);
    Ok(LoginResponse { token })
}