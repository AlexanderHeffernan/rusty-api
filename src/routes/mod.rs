/*
    This module is where all the routes for the API are defined.
    The 'configure_routes' function is called in the main function to set up the routes.
    Add new routes by creating a now file (e.g., `my_route.rs`) and registering it here.
*/

use actix_web::web;

pub mod admin_demo;
pub mod guest_demo;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(guest_demo::guest_demo)
       .service(admin_demo::admin_demo);
}