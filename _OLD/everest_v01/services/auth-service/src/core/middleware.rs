// src/core/middleware.rs
use crate::core::{
    errors::{AppError, AppResult},
    jwt::{Claims, JwtValidator},
};
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorUnauthorized,
    http::header::AUTHORIZATION,
    web,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{Ready, ready},
    rc::Rc,
};

// Middleware factory
pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
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
        let service = self.service.clone();

        Box::pin(async move {
            // Extract validator from app data
            let validator = req
                .app_data::<web::Data<JwtValidator>>()
                .ok_or_else(|| ErrorUnauthorized("JWT validator not configured"))?;

            // Extract token from Authorization header
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| ErrorUnauthorized("Missing authorization header"))?;

            let token = JwtValidator::extract_token_from_header(auth_header)
                .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

            // Validate token
            let claims = validator
                .validate_token(token)
                .await
                .map_err(|e| ErrorUnauthorized(e.to_string()))?;

            // Store claims in request extensions
            req.extensions_mut().insert(claims);

            // Continue with the request
            service.call(req).await
        })
    }
}

// Helper to extract claims from request
pub fn get_claims(req: &ServiceRequest) -> AppResult<Claims> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Claims not found in request".to_string()))
}

// Helper to extract claims from HttpRequest (for use in handlers)
pub fn extract_claims(req: &actix_web::HttpRequest) -> AppResult<Claims> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string()))
}

// Role-based authorization middleware
pub struct RequireRole {
    pub roles: Vec<String>,
}

impl RequireRole {
    pub fn new(roles: Vec<&str>) -> Self {
        Self {
            roles: roles.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn admin() -> Self {
        Self::new(vec!["admin"])
    }

    pub fn admin_or_partner() -> Self {
        Self::new(vec!["admin", "partner"])
    }

    pub fn admin_or_operator() -> Self {
        Self::new(vec!["admin", "operator"])
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireRoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireRoleMiddleware {
            service: Rc::new(service),
            roles: self.roles.clone(),
        }))
    }
}

pub struct RequireRoleMiddleware<S> {
    service: Rc<S>,
    roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RequireRoleMiddleware<S>
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
        let service = self.service.clone();
        let required_roles = self.roles.clone();

        Box::pin(async move {
            // Get claims from request extensions
            let claims = req
                .extensions()
                .get::<Claims>()
                .cloned()
                .ok_or_else(|| ErrorUnauthorized("User not authenticated"))?;

            // Check if user has any of the required roles
            let has_role = claims.roles.iter().any(|r| required_roles.contains(r));

            if !has_role {
                return Err(actix_web::error::ErrorForbidden("Insufficient permissions"));
            }

            service.call(req).await
        })
    }
}
