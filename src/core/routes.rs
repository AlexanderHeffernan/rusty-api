use actix_web::{web, HttpResponse, HttpMessage};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::core::auth::{login_user, register_user, validate_token, Claims};
use crate::core::user::{LoginInput, RegisterInput};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validate_token);

    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .service(
                web::scope("/protected")
                    .wrap(auth) // Apply the middleware
                    .route("/data", web::get().to(protected_data)),
            ),
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

async fn protected_data(req: actix_web::HttpRequest) -> HttpResponse {
    if let Some(claims) = req.extensions().get::<Claims>() {
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Access granted",
            "user": claims.sub,
        }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}