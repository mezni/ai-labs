use actix_web::{Error, HttpMessage, dev::ServiceRequest};
use actix_web_lab::middleware::Next;
use shared::jwt;
use std::future::{Ready, ready};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl actix_web_lab::middleware::Middleware<ServiceRequest> for AuthMiddleware {
    type Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest, next: Next) -> Self::Future {
        // Extract JWT token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth_header| {
                if auth_header.starts_with("Bearer ") {
                    Some(&auth_header[7..])
                } else {
                    None
                }
            });

        // For now, we'll just extract user ID from header for testing
        // In production, you'd validate JWT token
        let user_id = req
            .headers()
            .get(shared::constants::USER_ID_HEADER)
            .and_then(|header| header.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "anonymous".to_string());

        // Add user_id to request extensions for handlers to use
        req.extensions_mut().insert(user_id);

        ready(Ok(next.call(req)))
    }
}
