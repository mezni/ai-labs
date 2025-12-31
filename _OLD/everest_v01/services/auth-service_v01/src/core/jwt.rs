use crate::core::errors::AppError;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: String,
    pub roles: Vec<String>,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    pub fn is_partner(&self) -> bool {
        self.has_role("partner")
    }

    pub fn is_operator(&self) -> bool {
        self.has_role("operator")
    }

    pub fn can_access_network(&self, network_id: &str) -> bool {
        if self.is_admin() {
            return true;
        }
        self.network_id.as_ref().map_or(false, |nid| nid == network_id)
    }

    pub fn can_access_station(&self, station_id: &str) -> bool {
        if self.is_admin() {
            return true;
        }
        self.station_id.as_ref().map_or(false, |sid| sid == station_id)
    }
}

pub fn validate_token(token: &str, key: &DecodingKey, issuer: &str) -> Result<Claims, AppError> {
    let header = decode_header(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token header: {}", e)))?;

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[issuer]);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, key, &validation)
        .map_err(|e| AppError::Unauthorized(format!("Token validation failed: {}", e)))?;

    Ok(token_data.claims)
}

pub fn extract_bearer_token(auth_header: &str) -> Result<&str, AppError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Invalid authorization header format".to_string(),
        ));
    }
    Ok(&auth_header[7..])
}