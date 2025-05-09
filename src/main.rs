use rusty_api;

async fn password_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Password route accessed!")
}

async fn open_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Open route accessed!")
}

async fn get_role(_req: rusty_api::HttpRequest, user_id: i32) -> rusty_api::HttpResponse {
    rusty_api::get_user_field(user_id, "role").await
}

async fn set_role(req: rusty_api::HttpRequest, user_id: i32) -> rusty_api::HttpResponse {
    let new_role = req
        .uri()
        .query()
        .and_then(|q| q.split('&').find(|pair| pair.starts_with("args=")))
        .and_then(|pair| pair.split('=').nth(1))
        .unwrap_or("default_role");

    rusty_api::set_user_field(user_id, "role", &new_role).await
}

fn main() {
    let routes = rusty_api::Routes::new()
        .add_route_with_password("/password_route", password_route, "Password123")
        .add_route("/open_route", open_route)
        .add_route_with_auth("/get_role", get_role)
        .add_route_with_auth("/set_role", set_role);

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