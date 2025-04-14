/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This module is where all the routes for the API are defined.
    - The 'configure_routes' function is called in the main function to set up the routes.
    - Users can modify this file to add new routes.
    - For adding new routes, create a new file (e.g., `my_route.rs`) and register it here.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

use actix_web::web;

pub mod admin_demo;
pub mod guest_demo;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(guest_demo::guest_demo)
       .service(admin_demo::admin_demo);
}