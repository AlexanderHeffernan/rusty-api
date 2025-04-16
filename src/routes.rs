use actix_web::{web, Responder, FromRequest, HttpRequest, HttpResponse};
use actix_web::dev::Handler;
use crate::check_password;

pub struct Routes {
    routes: Vec<Box<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>,
}

impl Routes {
    /// Create a new `Routes` instance.
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a new route to the `Routes` instance with password protection.
    pub fn add_route_with_password<H, Args, R>(
        mut self,
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

    /// Add a new route to the `Routes` instance without password protection.
    pub fn add_route<H, Args, R>(mut self, path: &'static str, handler: H) -> Self
    where
        H: Handler<Args, Output = R> + Clone + Send + Sync + 'static,
        Args: FromRequest + 'static,
        R: Responder + 'static,
    {
        self.add_route_internal(path, handler, None)
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

    /// Apply the routes to a `ServiceConfig`.
    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        for route in &self.routes {
            route(cfg);
        }
    }
}