use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use super::auth::Claims;

pub struct RequireRole {
    required_role: String,
}

impl RequireRole {
    pub fn new(role: &str) -> Self {
        Self {
            required_role: role.to_string(),
        }
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
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct RequireRoleMiddleware<S> {
    service: Rc<S>,
    required_role: String,
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
        let srv = self.service.clone();
        let required_role = self.required_role.clone();

        Box::pin(async move {
            // Get claims from request extensions (set by JwtAuth middleware)
            let claims = req.extensions().get::<Claims>().cloned();

            tracing::debug!("RequireRole middleware - checking for role: {}", required_role);
            tracing::debug!("Claims present: {}", claims.is_some());

            match claims {
                Some(claims) => {
                    tracing::debug!("User has roles: {:?}", claims.roles);
                    
                    // Check if user has the required role
                    if claims.roles.iter().any(|r| r == &required_role) {
                        tracing::debug!("Role check passed for: {}", required_role);
                        srv.call(req).await
                    } else {
                        tracing::warn!(
                            "Role check failed. Required: '{}', User has: {:?}",
                            required_role,
                            claims.roles
                        );
                        Err(actix_web::error::ErrorForbidden(format!(
                            "Required role '{}' not found. User roles: {:?}",
                            required_role, claims.roles
                        )))
                    }
                }
                None => {
                    tracing::error!("No claims found in request extensions. JWT middleware may not have run.");
                    Err(actix_web::error::ErrorUnauthorized(
                        "Authentication required",
                    ))
                }
            }
        })
    }
}