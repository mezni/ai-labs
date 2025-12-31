use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};
use tokio::sync::RwLock;

// ============================================================================
// Claims Definition
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
    pub iss: String,
    pub email: String,
    pub preferred_username: String,
    #[serde(default)]
    pub roles: Vec<String>,
}

// ============================================================================
// JWKS Models
// ============================================================================
#[derive(Debug, Clone, Deserialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

// ============================================================================
// JWT Validator
// ============================================================================
pub struct JwtValidator {
    jwks_url: String,
    issuer: String,
    jwks_cache: Arc<RwLock<Option<JwksResponse>>>,
}

impl JwtValidator {
    pub fn new(jwks_url: String, issuer: String) -> Self {
        Self {
            jwks_url,
            issuer,
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    async fn fetch_jwks(&self) -> Result<JwksResponse, anyhow::Error> {
        let response = reqwest::get(&self.jwks_url).await?;
        let jwks: JwksResponse = response.json().await?;
        Ok(jwks)
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims, anyhow::Error> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or_else(|| anyhow::anyhow!("Token missing kid"))?;

        let jwks = {
            let cache = self.jwks_cache.read().await;
            if let Some(jwks) = cache.as_ref() {
                jwks.clone()
            } else {
                drop(cache);
                let jwks = self.fetch_jwks().await?;
                let mut cache = self.jwks_cache.write().await;
                *cache = Some(jwks.clone());
                jwks
            }
        };

        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| anyhow::anyhow!("No matching key found"))?;

        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);
        validation.validate_aud = false;

        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

// ============================================================================
// JWT Authentication Middleware
// ============================================================================
#[derive(Clone)]
pub struct JwtAuth {
    validator: Arc<JwtValidator>,
}

impl JwtAuth {
    pub fn new(jwks_url: String, issuer: String) -> Self {
        Self {
            validator: Arc::new(JwtValidator::new(jwks_url, issuer)),
        }
    }
}

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
            validator: self.validator.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
    validator: Arc<JwtValidator>,
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
        let srv = self.service.clone();
        let validator = self.validator.clone();

        Box::pin(async move {
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "));

            match token {
                Some(t) => match validator.validate_token(t).await {
                    Ok(claims) => {
                        req.extensions_mut().insert(claims);
                        srv.call(req).await
                    }
                    Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid Token")),
                },
                None => Err(actix_web::error::ErrorUnauthorized("Missing Authorization")),
            }
        })
    }
}

// ============================================================================
// Role-Based Authorization Middleware
// ============================================================================
#[derive(Clone)]
pub struct RequireAnyRole {
    allowed_roles: Vec<String>,
}

impl RequireAnyRole {
    pub fn new(roles: Vec<&str>) -> Self {
        Self {
            allowed_roles: roles.iter().map(|r| r.to_string()).collect(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireAnyRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAnyRoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireAnyRoleMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct RequireAnyRoleMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RequireAnyRoleMiddleware<S>
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
        let allowed = self.allowed_roles.clone();

        Box::pin(async move {
            let claims = req.extensions().get::<Claims>().cloned();
            if let Some(c) = claims {
                if c.roles.iter().any(|r| allowed.contains(r)) {
                    return srv.call(req).await;
                }
                return Err(actix_web::error::ErrorForbidden(
                    "Forbidden: Insufficient Role",
                ));
            }
            Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        })
    }
}

// ============================================================================
// Helper function to extract claims from request
// ============================================================================
use actix_web::HttpRequest;

pub fn get_claims(req: &HttpRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}