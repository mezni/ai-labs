use actix_web::{Error, HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    #[serde(rename = "typ")]
    pub token_type: String,
    pub azp: String,
    pub sid: String,
    pub acr: String,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Vec<String>,
    pub realm_access: RealmAccess,
    pub resource_access: HashMap<String, ResourceAccess>,
    pub scope: String,
    pub email_verified: bool,
    pub roles: Vec<String>,
    pub preferred_username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceAccess {
    pub roles: Vec<String>,
}

impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role) || self.realm_access.roles.iter().any(|r| r == role)
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }
}

pub async fn validate_token(
    token: &str,
    keycloak_url: &str,
    realm: &str,
) -> Result<Claims, AppError> {
    let jwks_url = format!(
        "{}/realms/{}/protocol/openid-connect/certs",
        keycloak_url, realm
    );
    let jwks_response = reqwest::get(&jwks_url)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fetch JWKS: {}", e)))?;

    let jwks: serde_json::Value = jwks_response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse JWKS: {}", e)))?;

    let header = decode_header(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token header: {}", e)))?;
    let kid = header
        .kid
        .ok_or_else(|| AppError::Unauthorized("No kid in token header".to_string()))?;

    let keys = jwks["keys"]
        .as_array()
        .ok_or_else(|| AppError::Internal("Invalid JWKS format".to_string()))?;
    let key = keys
        .iter()
        .find(|k| k["kid"].as_str() == Some(&kid))
        .ok_or_else(|| AppError::Unauthorized("Key not found in JWKS".to_string()))?;

    let n = key["n"]
        .as_str()
        .ok_or_else(|| AppError::Internal("Missing 'n' in JWKS key".to_string()))?;
    let e = key["e"]
        .as_str()
        .ok_or_else(|| AppError::Internal("Missing 'e' in JWKS key".to_string()))?;

    let decoding_key = DecodingKey::from_rsa_components(n, e)
        .map_err(|e| AppError::Internal(format!("Failed to create decoding key: {}", e)))?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["account"]);
    validation.set_issuer(&[format!("{}/realms/{}", keycloak_url, realm)]);

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::Unauthorized(format!("Token validation failed: {}", e)))?;

    Ok(token_data.claims)
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    let app_state = req.app_data::<actix_web::web::Data<crate::AppState>>();
    let app_state = match app_state {
        Some(state) => state,
        None => return Err((AppError::Internal("App state not found".into()).into(), req)),
    };

    match validate_token(
        token,
        &app_state.config.keycloak_url,
        &app_state.config.keycloak_realm,
    )
    .await
    {
        Ok(claims) => {
            let mut req = req;
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => Err((e.into(), req)),
    }
}

pub fn require_role(
    required_role: &'static str,
) -> impl Fn(
    ServiceRequest,
    BearerAuth,
)
    -> futures::future::LocalBoxFuture<'static, Result<ServiceRequest, (Error, ServiceRequest)>> {
    move |req, credentials| {
        Box::pin(async move {
            let req = jwt_validator(req, credentials).await?;

            // Borrow extensions safely
            let claims_opt = {
                let extensions_ref = req.extensions();
                extensions_ref.get::<Claims>().cloned()
            };

            match claims_opt {
                Some(claims) => {
                    if claims.has_role(required_role) {
                        Ok(req)
                    } else {
                        Err((
                            AppError::Forbidden(format!(
                                "Required role '{}' not found",
                                required_role
                            ))
                            .into(),
                            req,
                        ))
                    }
                }
                None => Err((
                    AppError::Unauthorized("Claims not found".into()).into(),
                    req,
                )),
            }
        })
    }
}
