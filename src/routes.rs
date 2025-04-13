use actix_web::{get, HttpRequest, HttpResponse, Responder};
use crate::auth::require_privilege;
use crate::models::PrivilegeLevel;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/admin")]
pub async fn admin(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {
    require_privilege(PrivilegeLevel::Admin)(&req)?;
    Ok(HttpResponse::Ok().body("Admin access granted"))
}