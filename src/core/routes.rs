use actix_web::{web, HttpResponse};
use crate::core::auth::{login_user, register_user};
use crate::core::user::{LoginInput, RegisterInput};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register)),
    );
}

async fn login(
    pool: web::Data<sqlx::SqlitePool>,
    input: web::Json<LoginInput>,
) -> HttpResponse {
    match login_user(&pool, input.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

async fn register(
    pool: web::Data<sqlx::SqlitePool>,
    input: web::Json<RegisterInput>,
) -> HttpResponse {
    match register_user(&pool, input.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}