use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PrivilegeLevel {
    Guest = 0,
    User = 1,
    Admin = 2,
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32, // INTEGER PRIMARY KEY AUTOINCREMENT
    pub email: String,
    pub api_key: String,
    pub privilege_level: i32,
}

impl User {
    pub fn privilege_level(&self) -> PrivilegeLevel {
        match self.privilege_level {
            0 => PrivilegeLevel::Guest,
            1 => PrivilegeLevel::User,
            2 => PrivilegeLevel::Admin,
            _ => PrivilegeLevel::Guest,
        }
    }
}