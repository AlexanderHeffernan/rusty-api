use actix_web::{get, HttpResponse, Responder};

/*
    This is a simple example of a guest route.
    It does not require any special privileges to access.
*/
#[get("/guest-demo")]
pub async fn guest_demo() -> impl Responder {
    HttpResponse::Ok().body("Guest endpoint")
}