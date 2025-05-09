/*!
 * The `routes` module provides functionality for defining and managing API routes.
 *
 * This module allows developers to create routes with or without password protection,
 * and apply them to an Actix Web `ServiceConfig`. It simplifies the process of setting
 * up API endpoints and ensures secure access to protected routes.
 *
 * This module features:
 * - **Password-Protected Routes**: Easily secure specific routes with a password.
 * - **Public Routes**: Define routes that are accessible without authentication.
 * - **Flexible Configuration**: Apply routes to an Actix Web `ServiceConfig` for seamless integration.
 *
 * The `Routes` struct serves as a container for all defined routes, allowing for
 * easy management and configuration.
 */
use actix_web::{web, Responder, FromRequest, HttpRequest, HttpResponse, dev::Handler};
use crate::core::auth::{validate_token, Claims};

/**
 * The `Routes` struct is used to manage API routes.
 *
 * It allows for the addition of routes with or without password protection,
 * and provides a method to apply these routes to an Actix Web `ServiceConfig`.
 *
 * # Example
 * ```rust
 * use rusty_api::Routes;
 * use actix_web::{HttpRequest, HttpResponse};
 * 
 * async fn public_route(_req: HttpRequest) -> HttpResponse {
 *     HttpResponse::Ok().body("Public route accessed!")
 * }
 * 
 * async fn protected_route(_req: HttpRequest) -> HttpResponse {
 *     HttpResponse::Ok().body("Protected route accessed!")
 * }
 * 
 * let routes = Routes::new()
 *     .add_route("/public", public_route)
 *     .add_route_with_password("/protected", protected_route, "SecretPassword");
 * ```
 */
pub struct Routes {
    routes: Vec<Box<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>,
}

impl Routes {
    /**
     * Create a new `Routes` instance.
     *
     * This initializes an empty collection of routes that can be configured
     * and applied to an Actix Web `ServiceConfig`, via the `Api` struct.
     *
     * # Example
     * ```rust
     * use rusty_api::Routes;
     * use rusty_api::Api;
     *
     * let routes = Routes::new();
     * let api = Api::new() 
     *     .configure_routes(routes);
     * ```
     */
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /**
     * Add a new route to the `Routes` instance with password protection.
     *
     * This method allows you to define a route that requires a password to access.
     * The password is passed as a query parameter in the request.
     *
     * # Arguments
     * - `path`: The URL path for the route.
     * - `handler`: The handler function for the route.
     * - `password`: The password required to access the route.
     *
     * # Example
     * ```rust
     * use rusty_api::{Routes, HttpRequest, HttpResponse};
     *
     * async fn protected_route(_req: HttpRequest) -> HttpResponse {
     *    HttpResponse::Ok().body("Protected route accessed!")
     * }
     *
     * let routes = Routes::new()
     *    .add_route_with_password("/protected", protected_route, "SecretPassword");
     * ```
     */
    pub fn add_route_with_password<H, Args, R>(
        self,
        path: &'static str,
        handler: H,
        password: &'static str,
    ) -> Self
    where
        H: Handler<Args, Output = R> + Clone + Send + Sync + 'static,
        Args: FromRequest + 'static,
        R: Responder + 'static,
    {
        self.add_route_internal(path, handler, Some(password))
    }

    /**
     * Add a new route to the `Routes` instance without password protection.
     *
     * This method allows you to define a public route that does not require authentication.
     *
     * # Arguments
     * - `path`: The URL path for the route.
     * - `handler`: The handler function for the route.
     *
     * # Example
     * ```rust
     * use rusty_api::{Routes, HttpRequest, HttpResponse};
     * 
     * async fn public_route(_req: HttpRequest) -> HttpResponse {
     *    HttpResponse::Ok().body("Public route accessed!")
     * }
     *
     * let routes = Routes::new()
     *   .add_route("/public", public_route);
     * ```
     */
    pub fn add_route<H, Args, R>(self, path: &'static str, handler: H) -> Self
    where
        H: Handler<Args, Output = R> + Clone + Send + Sync + 'static,
        Args: FromRequest + 'static,
        R: Responder + 'static,
    {
        self.add_route_internal(path, handler, None)
    }

    pub fn add_route_with_auth<H, R>(mut self, path: &'static str, handler: H) -> Self
    where
        H: Fn(HttpRequest, i32) -> R + Clone + Send + Sync + 'static,
        R: futures_util::Future<Output = HttpResponse> + 'static,
    {
        let wrapped_handler = move |req: HttpRequest| {
            let handler = handler.clone();
            async move {
                // Extract and validate the token
                let token = match req
                    .headers()
                    .get("Authorization")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| h.strip_prefix("Bearer "))
                {
                    Some(token) => token,
                    None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
                };

                // Validate the token and extract the user ID
                let user_id = match validate_token(token) {
                    Ok(claims) => claims.sub,
                    Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
                };

                // Call the handler with the user ID
                handler(req, user_id).await
            }
        };

        let route = {
            let wrapped_handler = wrapped_handler.clone(); // Clone the handler inside the closure
            move |cfg: &mut web::ServiceConfig| {
                cfg.route(path, web::get().to(wrapped_handler.clone())); // Clone again for Actix
            }
        };

        self.routes.push(Box::new(route));
        self
    }

    /// Internal function to handle adding routes with or without passwords.
    fn add_route_internal<H, Args, R>(
        mut self,
        path: &'static str,
        handler: H,
        password: Option<&'static str>,
    ) -> Self
    where
        H: Handler<Args, Output = R> + Clone + Send + Sync + 'static,
        Args: FromRequest + 'static,
        R: Responder + 'static,
    {
        let handler = handler.clone(); // Clone the handler to avoid moving it
        let wrapped_handler = move |req: HttpRequest, args: Args| {
            let handler = handler.clone(); // Clone the handler inside the closure
            async move {
                if let Some(expected_password) = password {
                    if !check_password(&req, expected_password) {
                        return HttpResponse::Unauthorized().body("Invalid password").into();
                    }
                }
                // Call the original handler and convert its output to an HttpResponse
                handler.call(args).await.respond_to(&req).map_into_boxed_body()
            }
        };

        let route = move |cfg: &mut web::ServiceConfig| {
            let wrapped_handler = wrapped_handler.clone(); // Clone the wrapped handler inside the route closure
            cfg.service(web::resource(path).route(web::get().to(wrapped_handler)));
        };
        self.routes.push(Box::new(route));
        self
    }

    /**
     * Apply the routes to a `ServiceConfig`.
     *
     * This method iterates over all defined routes and applies them to the
     * provided Axtix Web `ServiceConfig`. It is used internally by the `Api` struct.
     *
     * # Arguments
     * - `cfg`: A mutable reference to the `ServiceConfig` to which the routes will be applied.
     *
     * # Example
     * ```rust
     * use rusty_api::{Routes, Api};
     *
     * let routes = Routes::new();
     *
     * let api = Api::new()
     *    .configure_routes(routes); // The configure_routes method calls the configure method internally.
     * ```
     */
    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        for route in &self.routes {
            route(cfg);
        }
    }
}

/// Check if the request contains the expected password in the query string.
fn check_password(req: &HttpRequest, expected_password: &str) -> bool {
    let query_string = req.query_string();

    for pair in query_string.split('&') {
        let mut key_value = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
            if key == "password" && value == expected_password {
                return true;
            }
        }
    }

    false
}