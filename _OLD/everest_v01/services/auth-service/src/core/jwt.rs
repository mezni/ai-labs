// src/core/jwt.rs
use crate::core::{
    config::JwtConfig,
    errors::{AppError, AppResult},
};
use dashmap::DashMap;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Keycloak user ID
    pub user_id: String,    // Our internal user ID
    pub roles: Vec<String>, // User roles
    #[serde(default)]
    pub network_id: String, // Network ID for partners/operators
    #[serde(default)]
    pub station_id: String, // Station ID for operators
    pub iss: String,        // Issuer
    pub exp: usize,         // Expiration timestamp
    pub iat: usize,         // Issued at
    #[serde(default)]
    pub email: String, // User email
    #[serde(default)]
    pub preferred_username: String, // Username
}

#[derive(Clone)]
struct CachedKey {
    key: DecodingKey,
    cached_at: u64,
}

pub struct JwtValidator {
    config: JwtConfig,
    keys_cache: Arc<DashMap<String, CachedKey>>,
    http_client: reqwest::Client,
}

#[derive(Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    #[serde(rename = "use")]
    use_: Option<String>,
    n: String,
    e: String,
}

impl JwtValidator {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            config,
            keys_cache: Arc::new(DashMap::new()),
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn validate_token(&self, token: &str) -> AppResult<Claims> {
        // Decode header to get kid
        let header = decode_header(token)
            .map_err(|e| AppError::JwtError(format!("Invalid token header: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::JwtError("Token missing kid (key ID)".to_string()))?;

        // Get decoding key (from cache or fetch)
        let decoding_key = self.get_decoding_key(&kid).await?;

        // Set up validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.config.issuer]);
        validation.validate_exp = true;

        // Decode and validate token
        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| AppError::JwtError(format!("Token validation failed: {}", e)))?;

        Ok(token_data.claims)
    }

    async fn get_decoding_key(&self, kid: &str) -> AppResult<DecodingKey> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check cache
        if let Some(cached) = self.keys_cache.get(kid) {
            if now - cached.cached_at < self.config.cache_duration_secs {
                return Ok(cached.key.clone());
            }
        }

        // Fetch from JWKS endpoint
        let jwks: JwksResponse = self
            .http_client
            .get(&self.config.jwks_url)
            .send()
            .await
            .map_err(|e| AppError::JwtError(format!("Failed to fetch JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::JwtError(format!("Failed to parse JWKS: {}", e)))?;

        // Find the key
        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AppError::JwtError(format!("Key {} not found in JWKS", kid)))?;

        // Create decoding key
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AppError::JwtError(format!("Failed to create decoding key: {}", e)))?;

        // Cache it
        self.keys_cache.insert(
            kid.to_string(),
            CachedKey {
                key: decoding_key.clone(),
                cached_at: now,
            },
        );

        tracing::debug!("Cached new JWT key: {}", kid);

        Ok(decoding_key)
    }

    pub fn extract_token_from_header(auth_header: &str) -> AppResult<&str> {
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Invalid authorization header format".to_string(),
            ));
        }

        Ok(&auth_header[7..])
    }
}

// Helper to check if user has required role
impl Claims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        self.roles.iter().any(|r| roles.contains(&r.as_str()))
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

    pub fn is_user(&self) -> bool {
        self.has_role("user")
    }
}
