CREATE TABLE IF NOT EXISTS users (
    user_id         VARCHAR(32) PRIMARY KEY,
    keycloak_id     VARCHAR(255) UNIQUE NOT NULL,

    email           VARCHAR(255) UNIQUE NOT NULL,
    username        VARCHAR(100) UNIQUE NOT NULL,

    first_name      VARCHAR(100),
    last_name       VARCHAR(100),
    phone           VARCHAR(20),
    photo           TEXT,

    is_verified     BOOLEAN DEFAULT TRUE,
    role            VARCHAR(50) NOT NULL DEFAULT 'user',

    network_id      VARCHAR(32) NOT NULL DEFAULT '',
    station_id     VARCHAR(32) NOT NULL DEFAULT '',
    source          VARCHAR(20) NOT NULL DEFAULT 'web',

    is_active       BOOLEAN DEFAULT TRUE,
    deleted_at      TIMESTAMPTZ,
    last_login_at   TIMESTAMPTZ,

    created_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    created_by      VARCHAR(32),
    updated_by      VARCHAR(32),

    CONSTRAINT valid_role CHECK (
        role IN ('user', 'admin', 'partner', 'operator')
    ),
    CONSTRAINT valid_source CHECK (
        source IN ('web', 'mobile', 'internal')
    ),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE)
        OR
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);


CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id     VARCHAR(32) PRIMARY KEY,

    email               VARCHAR(255) NOT NULL,
    username            VARCHAR(100) NOT NULL,

    first_name          VARCHAR(100),
    last_name           VARCHAR(100),
    phone               VARCHAR(20),

    verification_token  VARCHAR(255) NOT NULL,
    status              VARCHAR(20) NOT NULL DEFAULT 'pending',

    keycloak_id         VARCHAR(255) UNIQUE NOT NULL,
    user_id             VARCHAR(32),

    resend_count        INTEGER DEFAULT 0,
    expires_at          TIMESTAMPTZ NOT NULL,
    verified_at         TIMESTAMPTZ,

    created_at          TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    ip_address          VARCHAR(50),
    user_agent          TEXT,
    source              VARCHAR(20) NOT NULL DEFAULT 'web',

    CONSTRAINT valid_registration_status CHECK (
        status IN ('pending', 'verified', 'expired', 'cancelled')
    ),
    CONSTRAINT fk_registration_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE SET NULL
);


Health & Status
    GET /health: Service health check (no auth)

Registration & Verification
    POST /register: Create new registration (no auth)
    POST /verify: Verify registration token (no auth)
    POST /verify/resend: Resend verification email (no auth)

Authentication
    POST /auth/login: User login (no auth)
    POST /auth/logout: User logout (auth required)


sequenceDiagram
    participant C as Client (Frontend)
    participant A as Auth Service
    participant K as Keycloak
    participant DB as Service Database
    participant J as Job/Cron Service

    %% ====================
    %% REGISTRATION FLOW
    %% ====================
    Note over C,A: 1. REGISTRATION FLOW
    C->>A: POST /api/v1/register
    A->>K: Check if user exists
    K-->>A: User not found
    A->>DB: Create user record (status: "pending")
    DB-->>A: User saved
    A->>K: Create user (enabled: false, emailVerified: false)
    K-->>A: User created in Keycloak
    A->>K: Send verification email
    A->>DB: Set verification expiry (24h)
    A-->>C: 201 Created - Verification email sent

    %% ====================
    %% VERIFICATION FLOWS
    %% ====================
    
    %% Successful verification
    Note over C,A: 2. EMAIL VERIFICATION (SUCCESS)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Still valid (<24h)
    A->>K: Verify email token/OTP
    K-->>A: Email verified
    A->>K: Enable user account
    A->>DB: Update user status to "active"
    A-->>C: 200 OK - Account activated

    %% Expired verification
    Note over C,A: 3. EMAIL VERIFICATION (EXPIRED)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Verification expired (>24h)
    A->>DB: Update status to "verification_expired"
    A->>K: Delete temporary user
    A-->>C: 410 Gone - Verification expired

    %% Resend verification
    Note over C,A: 4. RESEND VERIFICATION
    C->>A: POST /api/v1/register/resend
    A->>DB: Check last sent time & expiry
    DB-->>A: Can resend (within limits)
    A->>K: Resend verification email
    K-->>A: Email sent
    A->>DB: Update resend count & timestamp
    A-->>C: 200 OK - New verification sent

    %% ====================
    %% AUTHENTICATION FLOWS
    %% ====================

    %% Successful login (verified user)
    Note over C,A: 5. LOGIN FLOW (VERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user (OAuth2/OpenID)
    K-->>A: Return tokens (access, refresh, id_token)
    A->>DB: Store session/refresh token
    A-->>C: 200 OK with tokens

    %% Login attempt for unverified user
    Note over C,A: 6. LOGIN FLOW (UNVERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user
    K-->>A: Authentication successful but user disabled
    A->>DB: Check user status
    DB-->>A: Status = "pending_verification"
    A-->>C: 403 Forbidden - Account not verified



actix-cors = "0.7.1"
actix-web = "4.12.1"
anyhow = "1.0.100"
async-trait = "0.1.89"
chrono = { version = "0.4.42", features = ["serde"] }
dotenvy = "0.15.7"
nanoid = "0.4.0"
reqwest = { version = "0.12.26", features = ["json", "rustls-tls"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
sqlx = { version = "0.8.6", features = ["chrono", "macros", "postgres", "runtime-tokio-rustls"] }
thiserror = "2.0.17"
tokio = { version = "1.48.0", features = ["full"] }
tracing = "0.1.44"
tracing-actix-web = "0.7.20"
tracing-subscriber = { version = "0.3.22", features = ["env-filter"] }
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }
validator = { version = "0.20.0", features = ["derive"] }


HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgresql://postgres:password@localhost:5800/auth_db
# Options: error, warn, info, debug, trace
RUST_LOG=debug


# Keycloak Configuration
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_REALM=myrealm

# Frontend client (public, for user authentication)
KEYCLOAK_AUTH_CLIENT_ID=auth-client

# Backend service account (confidential, for admin operations)
KEYCLOAK_BACKEND_CLIENT_ID=backend-admin
KEYCLOAK_BACKEND_CLIENT_SECRET=backend-admin-secret

files: 
core: config.rs  constants.rs  database.rs  errors.rs  logging.rs  mod.rs
domain: entities.rs  enums.rs  mod.rs  repositories.rs (traits)  services.rs (traits) value_objects.rs
application: health_service.rs registration_service.rs authentication_service.rs dtos/authentication.rs dtos/health.rs  dtos/registration.rs
infrastructure:  keycloak_client.rs infrastructure/repositories/user_repo.rs infrastructure/repositories/registration_repo.rs
presentation: openapi.rs controllers/health_controller.rs controllers/registration_controller.rs controllers/authentication_controller.rs
main.rs lib.rs



use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

// ============================================================================
// Errors & Results
// ============================================================================

#[derive(Debug)]
pub enum AppError {
    KeycloakError(String),
    NetworkError(String),
    AuthenticationError(String),
    NotFound(String),
    Unauthorized(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeycloakError(m) => write!(f, "Keycloak: {}", m),
            Self::NetworkError(m) => write!(f, "Network: {}", m),
            Self::AuthenticationError(m) => write!(f, "Auth: {}", m),
            Self::NotFound(m) => write!(f, "Not Found: {}", m),
            Self::Unauthorized(m) => write!(f, "Unauthorized: {}", m),
        }
    }
}

impl std::error::Error for AppError {}
pub type AppResult<T> = Result<T, AppError>;

// ============================================================================
// Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub enabled: bool,
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub enabled: bool,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub credentials: Vec<Credential>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Credential {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
    pub temporary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoleMapping {
    pub id: String,
    pub name: String,
}

// ============================================================================
// Trait Definition
// ============================================================================

#[async_trait]
pub trait KeycloakClient: Send + Sync {
    async fn create_user(
        &self,
        email: &str,
        username: &str,
        password: &str,
        attributes: Option<HashMap<String, Vec<String>>>,
    ) -> AppResult<String>;
    async fn get_user(&self, user_id: &str) -> AppResult<KeycloakUser>;
    async fn enable_user(&self, user_id: &str) -> AppResult<()>;
    async fn disable_user(&self, user_id: &str) -> AppResult<()>;
    async fn assign_role(&self, user_id: &str, role_name: &str) -> AppResult<()>;
    async fn authenticate(&self, username: &str, password: &str) -> AppResult<TokenResponse>;
    async fn refresh_token(&self, refresh_token: &str) -> AppResult<TokenResponse>;
    async fn logout(&self, refresh_token: &str) -> AppResult<()>;
    async fn verify_token(&self, access_token: &str) -> AppResult<serde_json::Value>;
    async fn get_user_info(&self, access_token: &str) -> AppResult<serde_json::Value>;
    async fn send_verification_email(&self, keycloak_id: &str) -> AppResult<()>;
}

// ============================================================================
// Implementation
// ============================================================================

pub struct HttpKeycloakClient {
    base_url: String, // e.g., "http://localhost:8080"
    realm: String,
    backend_id: String,
    backend_secret: String,
    auth_id: String,
    http: reqwest::Client,
    admin_token_cache: RwLock<Option<AdminToken>>,
}

#[derive(Clone)]
struct AdminToken {
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl HttpKeycloakClient {
    pub fn new(
        base_url: String,
        realm: String,
        backend_id: String,
        backend_secret: String,
        auth_id: String,
    ) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            realm,
            backend_id,
            backend_secret,
            auth_id,
            http: reqwest::Client::new(),
            admin_token_cache: RwLock::new(None),
        }
    }

    // --- URL Helper Methods (Inspired by your code) ---

    fn auth_url(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect",
            self.base_url, self.realm
        )
    }

    fn admin_url(&self) -> String {
        format!("{}/admin/realms/{}", self.base_url, self.realm)
    }

    fn token_endpoint(&self) -> String {
        format!("{}/token", self.auth_url())
    }

    fn user_endpoint(&self, user_id: &str) -> String {
        format!("{}/users/{}", self.admin_url(), user_id)
    }

    fn role_endpoint(&self, role_name: &str) -> String {
        format!("{}/roles/{}", self.admin_url(), role_name)
    }

    async fn get_admin_token(&self) -> AppResult<String> {
        {
            let cache = self.admin_token_cache.read().await;
            if let Some(ref t) = *cache {
                if chrono::Utc::now() < t.expires_at {
                    return Ok(t.token.clone());
                }
            }
        }

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.backend_id.as_str()),
            ("client_secret", self.backend_secret.as_str()),
        ];

        let resp = self
            .http
            .post(self.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::AuthenticationError(format!(
                "Admin login failed: {}",
                body
            )));
        }

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        let token = data["access_token"]
            .as_str()
            .ok_or(AppError::AuthenticationError("No access token".into()))?
            .to_string();
        let expires_in = data["expires_in"].as_i64().unwrap_or(300);

        let mut cache = self.admin_token_cache.write().await;
        *cache = Some(AdminToken {
            token: token.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(expires_in - 20),
        });

        Ok(token)
    }
}

#[async_trait]
impl KeycloakClient for HttpKeycloakClient {
    async fn create_user(
        &self,
        email: &str,
        username: &str,
        password: &str,
        attributes: Option<HashMap<String, Vec<String>>>,
    ) -> AppResult<String> {
        let token = self.get_admin_token().await?;
        let body = CreateUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            enabled: true,
            email_verified: true,
            credentials: vec![Credential {
                kind: "password".into(),
                value: password.into(),
                temporary: false,
            }],
            attributes,
        };

        let url = format!("{}/users", self.admin_url());
        let resp = self
            .http
            .post(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::KeycloakError(
                resp.text().await.unwrap_or_default(),
            ));
        }

        let loc = resp
            .headers()
            .get("Location")
            .and_then(|l| l.to_str().ok())
            .ok_or(AppError::KeycloakError(
                "No user ID in Location header".into(),
            ))?;
        Ok(loc.split('/').last().unwrap_or_default().to_string())
    }

    async fn get_user(&self, user_id: &str) -> AppResult<KeycloakUser> {
        let token = self.get_admin_token().await?;
        let resp = self
            .http
            .get(self.user_endpoint(user_id))
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() == 404 {
            return Err(AppError::NotFound("User not found".into()));
        }
        resp.json::<KeycloakUser>()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }

    async fn enable_user(&self, user_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;
        let mut body = HashMap::new();
        body.insert("enabled", true);

        let resp = self
            .http
            .put(self.user_endpoint(user_id))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::KeycloakError(
                resp.text().await.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    async fn disable_user(&self, user_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;
        let mut body = HashMap::new();
        body.insert("enabled", false);

        let resp = self
            .http
            .put(self.user_endpoint(user_id))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::KeycloakError(
                resp.text().await.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    async fn assign_role(&self, user_id: &str, role_name: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        let role_resp = self
            .http
            .get(self.role_endpoint(role_name))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !role_resp.status().is_success() {
            let body = role_resp.text().await.unwrap_or_default();
            return Err(AppError::NotFound(format!(
                "Role {} lookup failed: {}",
                role_name, body
            )));
        }

        let role_data: RoleMapping = role_resp
            .json()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        let mapping_url = format!("{}/role-mappings/realm", self.user_endpoint(user_id));

        let resp = self
            .http
            .post(mapping_url)
            .bearer_auth(token)
            .json(&vec![role_data])
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::KeycloakError(
                resp.text().await.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    async fn authenticate(&self, username: &str, password: &str) -> AppResult<TokenResponse> {
        let params = [
            ("grant_type", "password"),
            ("client_id", self.auth_id.as_str()),
            ("username", username),
            ("password", password),
        ];

        let resp = self
            .http
            .post(self.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::AuthenticationError("Login failed".into()));
        }
        resp.json::<TokenResponse>()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }

    async fn refresh_token(&self, refresh_token: &str) -> AppResult<TokenResponse> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", self.auth_id.as_str()),
            ("refresh_token", refresh_token),
        ];

        let resp = self
            .http
            .post(self.token_endpoint())
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::Unauthorized("Refresh failed".into()));
        }
        resp.json::<TokenResponse>()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }

    async fn logout(&self, refresh_token: &str) -> AppResult<()> {
        let url = format!("{}/logout", self.auth_url());
        let params = [
            ("client_id", self.auth_id.as_str()),
            ("refresh_token", refresh_token),
        ];
        let _ = self.http.post(url).form(&params).send().await;
        Ok(())
    }

    async fn verify_token(&self, access_token: &str) -> AppResult<serde_json::Value> {
        let url = format!("{}/token/introspect", self.auth_url());
        let params = [
            ("token", access_token),
            ("client_id", self.auth_id.as_str()),
        ];

        let resp = self
            .http
            .post(url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if data["active"].as_bool() == Some(true) {
            Ok(data)
        } else {
            Err(AppError::Unauthorized("Inactive token".into()))
        }
    }

    async fn get_user_info(&self, access_token: &str) -> AppResult<serde_json::Value> {
        let url = format!("{}/userinfo", self.auth_url());
        let resp = self
            .http
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::Unauthorized("Invalid token".into()));
        }
        resp.json()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }

    async fn send_verification_email(&self, keycloak_id: &str) -> AppResult<()> {
        let token = self.get_admin_token().await?;

        // Keycloak Admin API endpoint to trigger specific actions (like email verification)
        let url = format!("{}/execute-actions-email", self.user_endpoint(keycloak_id));

        // We send a PUT request with the action we want the user to perform
        let actions = vec!["VERIFY_EMAIL"];

        let resp = self
            .http
            .put(url)
            .bearer_auth(token)
            .json(&actions)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::KeycloakError(format!(
                "Failed to send email: {}",
                body
            )));
        }

        Ok(())
    }
}

