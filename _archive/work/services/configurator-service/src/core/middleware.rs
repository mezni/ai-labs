use actix_web::{Error, HttpMessage};
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};
use thiserror::Error;

// === ERROR TYPES ===
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Missing token")]
    MissingToken,
}

impl actix_web::ResponseError for AuthError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AuthError::InvalidToken => {
                actix_web::HttpResponse::Unauthorized().json("Invalid token")
            }
            AuthError::TokenExpired => {
                actix_web::HttpResponse::Unauthorized().json("Token expired")
            }
            AuthError::InsufficientPermissions => {
                actix_web::HttpResponse::Forbidden().json("Insufficient permissions")
            }
            AuthError::MissingToken => {
                actix_web::HttpResponse::Unauthorized().json("Missing token")
            }
        }
    }
}

// === JWT CLAIMS ===
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // username
    pub role: String, // user role
    pub exp: u64,     // expiry timestamp
}

// === AUTHENTICATED USER ===
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub username: String,
    pub role: String,
}

impl actix_web::FromRequest for AuthenticatedUser {
    type Error = AuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // ✅ Load environment variables (ensures JWT_SECRET is available)
        dotenv().ok();

        // ✅ Load JWT secret from environment (no hardcoded fallback)
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET not set in environment"))
            .trim()
            .to_string();

        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        // ✅ Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| token.trim().to_string());

        let token = match token {
            Some(token) => token,
            None => return ready(Err(AuthError::MissingToken)),
        };

        // ✅ Verify and decode the token
        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(token_data) => ready(Ok(AuthenticatedUser {
                username: token_data.claims.sub,
                role: token_data.claims.role,
            })),
            Err(e) => {
                let auth_error = match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    _ => AuthError::InvalidToken,
                };
                ready(Err(auth_error))
            }
        }
    }
}

// === ROLE-BASED ACCESS CONTROL ===
pub fn require_role(user: &AuthenticatedUser, required_role: &str) -> Result<(), AuthError> {
    if user.role != required_role && user.role != "admin" {
        return Err(AuthError::InsufficientPermissions);
    }
    Ok(())
}
