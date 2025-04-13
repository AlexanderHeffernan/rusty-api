use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest,
};
use futures_util::future::{LocalBoxFuture, Ready};
use sqlx::SqlitePool;
use std::rc::Rc;

use crate::models::{PrivilegeLevel, User};

pub struct Auth {
    db_pool: SqlitePool,
}

impl Auth {
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

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(AuthMiddleware {
            service: Rc::new(service),
            db_pool: self.db_pool.clone(),
        })
    }
}

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

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db_pool = self.db_pool.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let api_key = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer ").map(str::to_string));

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

            let privilege_level = user
                .as_ref()
                .map(|u| u.privilege_level())
                .unwrap_or(PrivilegeLevel::Guest);
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(privilege_level);

            service.call(req).await
        })
    }
}

pub fn require_privilege(min_level: PrivilegeLevel) -> impl Fn(&HttpRequest) -> Result<(), actix_web::Error> {
    move |req: &HttpRequest| {
        let current_level = req
            .extensions()
            .get::<PrivilegeLevel>()
            .copied()
            .unwrap_or(PrivilegeLevel::Guest);
        if current_level as i32 >= min_level as i32 {
            Ok(())
        } else {
            Err(actix_web::error::ErrorForbidden("Insufficient privileges"))
        }
    }
}