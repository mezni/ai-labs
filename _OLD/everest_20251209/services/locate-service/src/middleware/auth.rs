use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::{
    future::{Ready, ready},
    rc::Rc,
    sync::Arc,
};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub iss: String,
    #[serde(rename = "typ")]
    pub token_type: String,
    pub azp: String,
    pub sid: String,
    pub scope: String,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub roles: Vec<String>,
}

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
        // Decode header to get kid
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("Token missing kid"))?;

        // Get JWKS (from cache or fetch)
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

        // Find matching key
        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| anyhow::anyhow!("No matching key found"))?;

        // Create decoding key from JWK
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

        // Validate token
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

        Ok(token_data.claims)
    }
}

// Middleware factory
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
            // Extract token from Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "));

            let token = match auth_header {
                Some(t) => t,
                None => {
                    return Err(actix_web::error::ErrorUnauthorized(
                        "Missing or invalid authorization header",
                    ));
                }
            };

            // Validate token
            let claims = validator.validate_token(token).await.map_err(|e| {
                actix_web::error::ErrorUnauthorized(format!("Token validation failed: {}", e))
            })?;

            // Store claims in request extensions
            req.extensions_mut().insert(claims);

            // Call the next service
            srv.call(req).await
        })
    }
}

// Helper to extract claims from request
pub fn extract_claims(req: &actix_web::HttpRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}
