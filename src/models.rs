/*
    This module defines the data models used in the api server.
    Add privilege levels and user fields here.
    Note:
    - Ensure that any changes to the 'User' struct match the database schema.
*/

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use num_enum::{IntoPrimitive, TryFromPrimitive}; // Import num_enum traits
use std::convert::TryFrom;

/* Represents the privilege levels for users in the system. */
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum PrivilegeLevel {
    Guest = 0, // Default privilege level for unauthenticated users
    User = 1, // Standard privilege level for authenticated users
    Admin = 2, // Highest privilege level for administrators
    // Add new privilege levels below
    // Example: SuperAdmin = 3,
}

/* Represents a user in the database. */
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32, // Unique identifier for the user (primary key in the database)
    pub email: String, // Email address of the user
    pub api_key: String, // API key associated with the user for authentication
    pub privilege_level: i32, // Privilege level of the user (stored as an integer in the database)
    // Add new fields as needed (ensure they are also in the database)
    // Example: pub name: String,
}

impl User {
    /*
        Converts the integer privilege level from the database into the 'PrivilegeLevel' enum.
        # Returns
        - A 'PrivilegeLevel' enum representing the user's privilege level.
    */
    pub fn privilege_level(&self) -> PrivilegeLevel {
        PrivilegeLevel::try_from(self.privilege_level).unwrap_or(PrivilegeLevel::Guest)
    }
}