use actix_web::web;

pub mod admin_demo;
pub mod guest_demo;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(guest_demo::guest_demo)
       .service(admin_demo::admin_demo);
}