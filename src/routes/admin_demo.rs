/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This module defines a simple admin route for demonstration purposes.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

use actix_web::{get, HttpRequest, HttpResponse, Responder};
use crate::core::auth::require_privilege;
use crate::models::PrivilegeLevel;

/*
    This is a simple example of an admin route.
    It checks if the user has admin privileges before granting access.
*/
#[get("/admin-demo")]
pub async fn admin_demo(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {
    require_privilege(PrivilegeLevel::Admin)(&req)?;
    Ok(HttpResponse::Ok().body("Admin access granted"))
}