/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This module defines a simple guest route for demonstration purposes.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/
use actix_web::{get, HttpResponse, Responder};

/*
    This is a simple example of a guest route.
    It does not require any special privileges to access.
*/
#[get("/guest-demo")]
pub async fn guest_demo() -> impl Responder {
    HttpResponse::Ok().body("Guest endpoint")
}