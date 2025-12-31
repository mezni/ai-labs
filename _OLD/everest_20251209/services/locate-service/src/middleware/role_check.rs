use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use std::{
    future::{Ready, ready},
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

            match claims {
                Some(claims) => {
                    // Check if user has the required role
                    if claims.roles.iter().any(|r| r == &required_role) {
                        srv.call(req).await
                    } else {
                        Err(actix_web::error::ErrorForbidden(format!(
                            "Required role '{}' not found. User roles: {:?}",
                            required_role, claims.roles
                        )))
                    }
                }
                None => Err(actix_web::error::ErrorUnauthorized(
                    "Authentication required",
                )),
            }
        })
    }
}
