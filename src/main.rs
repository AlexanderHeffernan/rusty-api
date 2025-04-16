use rusty_api;

async fn helloadmin(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Hello admin from a dedicated route!")
}

async fn hello(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Hello from a dedicated route!")
}

fn main() {
    let routes = rusty_api::Routes::new()
        .add_route_with_password("/helloadmin", helloadmin, "Password123")
        .add_route("/hello", hello);

    rusty_api::Api::new()
        .certs("certs/cert.pem", "certs/key.pem")
        .auth_db("users.db")
        .rate_limit(3, 20)
        .bind("127.0.0.1", 8443)
        .configure_routes(routes)
        .configure_cors(|| {
            rusty_api::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allowed_header("ngrok-skip-browser-warning")
        })
        .start();
}