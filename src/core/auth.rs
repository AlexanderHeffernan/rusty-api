/*
    Author: Alexander Heffernan
    This file is part of the Rust API Template.

    Description:
    - This module implements authentication middleware for Actix Web.
    - It checks for a valid API key in the request headers.
    - If the key is valid, it retrieves the user information from the database
      and sets the user and privilege level in the request extensions.

     License:
    - This code is provided as-is, without warranty of any kind.
    - You are free to use, modify, and distribute this code as part of your projects.
*/

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest,
};
use futures_util::future::{LocalBoxFuture, Ready};
use sqlx::SqlitePool;
use std::rc::Rc;

use crate::models::{PrivilegeLevel, User};

/*
    Authentication middleware for Actix Web.
    This middleware checks for a valid API key in the request headers.
    If the key is valid, it retrieves the user information from the database
    and sets the user and privilege level in the request extensions.
    If the key is invalid or missing, it sets the privilege level to Guest.
    The middleware also provides a function to check if the user has the required privilege level.
*/
pub struct Auth {
    db_pool: SqlitePool,
}

impl Auth {
    // Creates a new instance of the 'Auth' middleware.
    pub fn new(db_pool: SqlitePool) -> Self {
        Auth { db_pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Creates a new instance of the 'AuthMiddleware' for the given service.
    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(AuthMiddleware {
            service: Rc::new(service),
            db_pool: self.db_pool.clone(),
        })
    }
}

// Middleware implementation for handling authentication.
pub struct AuthMiddleware<S> {
    service: Rc<S>,
    db_pool: SqlitePool,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Forward readiness checks to the inner service.
    forward_ready!(service);

    // Handles incoming requests, check for API key, and sets user context.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract the API key from the 'Authorization' header.
            let api_key = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer ").map(str::to_string));

            // Query the database for the user associated with the API key.
            let user = match api_key {
                Some(key) => {
                    sqlx::query_as::<_, User>("SELECT * FROM users WHERE api_key = ?")
                        .bind(key)
                        .fetch_optional(&db_pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorUnauthorized(format!("Database error: {}", e)))?
                }
                None => None,
            };

            // Determine the user's privilege level (default to Guest if no user is found).
            let privilege_level = user
                .as_ref()
                .map(|u| u.privilege_level())
                .unwrap_or(PrivilegeLevel::Guest);

            // Store the user and privilege level in the request's extensions for later use.
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(privilege_level);

            // Pass the request to the next service in the chain.
            service.call(req).await
        })
    }
}

/*
    Helper function to enforce privilege levels on routes.
    # Arguments
    - `min_level`: The minimum privilege level required to access the route.
    # Returns
    - A closure that takes a reference to the HttpRequest and returns a Result.
*/
pub fn require_privilege(min_level: PrivilegeLevel) -> impl Fn(&HttpRequest) -> Result<(), actix_web::Error> {
    move |req: &HttpRequest| {
        // Retrieve the current privilege level from the request's extensions.
        let current_level = req
            .extensions()
            .get::<PrivilegeLevel>()
            .copied()
            .unwrap_or(PrivilegeLevel::Guest);

        // Check if the current privilege level meets the required minimum.
        if current_level as i32 >= min_level as i32 {
            Ok(())
        } else {
            Err(actix_web::error::ErrorForbidden("Insufficient privileges"))
        }
    }
}