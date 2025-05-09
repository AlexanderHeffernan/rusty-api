use rusty_api;

async fn password_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Password route accessed!")
}

async fn open_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Open route accessed!")
}

async fn get_role(req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    let claims = match rusty_api::validate_token(&req) {
        Ok(claims) => claims,
        Err(err) => return err.into(),
    };

    match rusty_api::get_user_field(claims.sub, "role").await {
        Ok(Some(role)) => rusty_api::HttpResponse::Ok().body(role),
        Ok(None) => rusty_api::HttpResponse::NotFound().body("Role not found"),
        Err(_) => rusty_api::HttpResponse::InternalServerError().body("Database error"),
    }
}

fn main() {
    rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider());
    dotenv::dotenv().ok();
    
    let routes = rusty_api::Routes::new()
        .add_route_with_password("/password_route", password_route, "Password123")
        .add_route("/open_route", open_route)
        .add_route("/get_role", get_role);

    rusty_api::Api::new()
        .certs("certs/cert.pem", "certs/key.pem")
        .rate_limit(3, 20)
        .bind("127.0.0.1", 8443)
        .configure_routes(routes)
        .configure_cors(|| {
            rusty_api::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allowed_header("ngrok-skip-browser-warning")
        })
        .enable_user_db()
        .start();
}