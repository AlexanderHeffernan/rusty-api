use actix_web::{web, Responder, FromRequest};

pub struct Routes {
    routes: Vec<Box<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>,
}

impl Routes {
    /// Create a new `Routes` instance.
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a new route to the `Routes` instance.
    ///
    /// # Arguments
    /// * `path` - The path for the route.
    /// * `handler` - The handler function for the route.
    pub fn add_route<H, Args, R>(mut self, path: &'static str, handler: H) -> Self
    where
        H: actix_web::dev::Handler<Args, Output = R> + Clone + Send + Sync + 'static,
        Args: FromRequest + 'static,
        R: Responder + 'static,
    {
        let route = move |cfg: &mut web::ServiceConfig| {
            cfg.service(web::resource(path).route(web::get().to(handler.clone())));
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