use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub email: String,
    pub roles: Vec<String>,
    pub exp: i64,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    
    // In a real implementation, you would validate the token here
    // For now, we'll decode it without validation (for development only)
    match decode_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Invalid token"),
            req,
        )),
    }
}

fn decode_token(token: &str) -> Result<TokenClaims, String> {
    // Split JWT token
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    // Decode payload (second part)
    let payload = parts[1];
    let decoded = base64_decode(payload).map_err(|e| e.to_string())?;
    let json_str = String::from_utf8(decoded).map_err(|e| e.to_string())?;
    
    serde_json::from_str::<TokenClaims>(&json_str).map_err(|e| e.to_string())
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    URL_SAFE_NO_PAD.decode(input).map_err(|e| e.to_string())
}

pub fn extract_claims(req: &ServiceRequest) -> Option<TokenClaims> {
    req.extensions().get::<TokenClaims>().cloned()
}

pub fn is_admin(claims: &TokenClaims) -> bool {
    claims.roles.contains(&"admin".to_string())
}