/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This is the main entry point for the Rust API server.
    - Users can modify this file to add middleware or configure the server.
    - For adding new routes, use the `routes/` directory.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

use rusty_api::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}