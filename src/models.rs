use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/* Represents the privilege levels for users in the system. */
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PrivilegeLevel {
    Guest = 0, // Default privilege level for unauthenticated users
    User = 1, // Standard privilege level for authenticated users
    Admin = 2, // Highest privilege level for administrators
}

/* Represents a user in the database. */
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32, // Unique identifier for the user (primary key in the database)
    pub email: String, // Email address of the user
    pub api_key: String, // API key associated with the user for authentication
    pub privilege_level: i32, // Privilege level of the user (stored as an integer in the database)
}

impl User {
    /*
        Converts the integer privilege level from the database into the 'PrivilegeLevel' enum.
        # Returns
        - A 'PrivilegeLevel' enum representing the user's privilege level.
    */
    pub fn privilege_level(&self) -> PrivilegeLevel {
        match self.privilege_level {
            0 => PrivilegeLevel::Guest,
            1 => PrivilegeLevel::User,
            2 => PrivilegeLevel::Admin,
            _ => PrivilegeLevel::Guest,
        }
    }
}