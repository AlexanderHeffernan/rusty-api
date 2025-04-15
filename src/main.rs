use rusty_api;

async fn hello() -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Hello from a dedicated route!")
}

fn main() {
    let routes = rusty_api::Routes::new()
        .add_route("/hello", hello);

    rusty_api::Api::new()
        .certs("certs/cert.pem", "certs/key.pem")
        .auth_db("users.db")
        .rate_limit(3, 20)
        .bind("127.0.0.1", 8443)
        .configure_routes(routes)
        .start();
}