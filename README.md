# Rusty API
Rusty API is a secure and lightweight Rust library for building backend APIs. It features **HTTPS**, **password-protected routes**, **rate limiting**, and more, making it ideal for rapid API development.

## Features
- **Password-Protected Routes**: Easily secure specific routes with passwords.
- **HTTPS Support**: Built-in support for secure communication using Rustls.
- **Rate Limiting**: Prevent abuse with configurable rate limits.
- **CORS Configuration**: Flexible CORS settings for cross-origin requests.
- **Actix Web Integration**: Built on top of Actix Web for high performance.

## Installation
Add `rusty-api` to your `Cargo.toml`:
```toml
[dependencies]
rusty-api = "0.2.1"
```

## Usage
### Setting Up Your API
Here's an example of how to use rusty-api to create an API with public and password-protected routes:
```rust
use rusty_api;

async fn password_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Password route accessed!")
}

async fn open_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    rusty_api::HttpResponse::Ok().body("Open route accessed!")
}

fn main() {
    let routes = rusty_api::Routes::new()
        .add_route_with_password("/password_route", password_route, "Password123")
        .add_route("/open_route", open_route);

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
```

### Generating Self-Signed Certificates
To enable HTTPS, generate self-signed certificates:
```bash
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem
```

### Running the API
Run your API with:
```bash
cargo run
```

## Projects Using This Package
Here are some projects that use [rusty-api](https://crates.io/crates/rusty-api) for their API's. Want to add your project? See [Contributing](#contributing) below!

| Project | Description | Links |
|---------|-------------|-------|
| **ServerWatch** | A lightweight system monitoring tool that collects metrics and exposes them via a secure HTTPS endpoint. Built for Raspberry Pi and other Linux systems. | [GitHub](https://github.com/AlexanderHeffernan/ServerWatch) \| [Website](https://alexanderheffernan.github.io/ServerWatch/) |
| **Your Project Here** | Have a project using this package? Submit a PR to add it! | See Contributing below! |

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License.
