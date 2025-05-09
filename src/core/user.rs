/*!
 * User module
 * 
 * This module defines the many structs related to user management, including
 * user registration, login, and the user model itself.
 */
use serde::{Deserialize, Serialize};

/**
 * User struct
 *
 * This struct represents a user in the system, containing fields for the user's
 * ID, username, and password hash. The user database can contain more fields, but
 * these are the essential ones for authentication and identification.
 */
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

/**
 * Input struct for user registration
 *
 * This struct is used to deserialize the input data for user registration.
 * It contains fields for the username and password.
 */
#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

/**
 * Input struct for user login
 *
 * This struct is used to deserialize the input data for user login.
 * It contains fields for the username and password.
 */
#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}


/**
 * Response struct for user registration
 *
 * This struct is used to serialize the response data for user registration.
 * It contains a field for the users token, which is used for authentication.
 */
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}