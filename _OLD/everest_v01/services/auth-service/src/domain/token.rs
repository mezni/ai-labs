// src/domain/token.rs
use crate::core::{AppError, errors::AppResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Token types for various authentication flows
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
    PasswordReset,
    EmailVerification,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Access => "access",
            Self::Refresh => "refresh",
            Self::PasswordReset => "password_reset",
            Self::EmailVerification => "email_verification",
        }
    }

    pub fn default_expiration_duration(&self) -> Duration {
        match self {
            Self::Access => Duration::minutes(15),
            Self::Refresh => Duration::days(7),
            Self::PasswordReset => Duration::hours(1),
            Self::EmailVerification => Duration::hours(24),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Keycloak token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl TokenResponse {
    pub fn is_expired(&self) -> bool {
        self.expires_in <= 0
    }
}

/// Password reset token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub token_id: String,
    pub user_id: String,
    pub email: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

impl PasswordResetToken {
    pub fn new(
        token_id: String,
        user_id: String,
        email: String,
        token: String,
        ip_address: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + TokenType::PasswordReset.default_expiration_duration();

        Self {
            token_id,
            user_id,
            email,
            token,
            expires_at,
            used_at: None,
            created_at: now,
            ip_address,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.used_at.is_none() && !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn mark_used(&mut self) -> AppResult<()> {
        if !self.is_valid() {
            return Err(AppError::BadRequest("Token is not valid".to_string()));
        }

        self.used_at = Some(Utc::now());
        Ok(())
    }

    pub fn validate(&self, provided_token: &str) -> AppResult<()> {
        if !self.is_valid() {
            return Err(AppError::Unauthorized(
                "Token is expired or already used".to_string(),
            ));
        }

        if self.token != provided_token {
            return Err(AppError::Unauthorized("Invalid token".to_string()));
        }

        Ok(())
    }
}

/// Login credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

impl LoginCredentials {
    pub fn new(username: String, password: String) -> AppResult<Self> {
        if username.trim().is_empty() {
            return Err(AppError::Validation("Username cannot be empty".to_string()));
        }

        if password.is_empty() {
            return Err(AppError::Validation("Password cannot be empty".to_string()));
        }

        Ok(Self { username, password })
    }
}

/// Password change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangeRequest {
    pub old_password: String,
    pub new_password: String,
}

impl PasswordChangeRequest {
    pub fn new(old_password: String, new_password: String) -> AppResult<Self> {
        if old_password.is_empty() {
            return Err(AppError::Validation(
                "Old password cannot be empty".to_string(),
            ));
        }

        if new_password.is_empty() {
            return Err(AppError::Validation(
                "New password cannot be empty".to_string(),
            ));
        }

        if new_password.len() < crate::core::constants::MIN_PASSWORD_LENGTH {
            return Err(AppError::Validation(format!(
                "Password must be at least {} characters long",
                crate::core::constants::MIN_PASSWORD_LENGTH
            )));
        }

        if old_password == new_password {
            return Err(AppError::Validation(
                "New password must be different from old password".to_string(),
            ));
        }

        Ok(Self {
            old_password,
            new_password,
        })
    }
}

/// Password reset request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub token: String,
    pub new_password: String,
}

impl PasswordResetRequest {
    pub fn new(token: String, new_password: String) -> AppResult<Self> {
        if token.trim().is_empty() {
            return Err(AppError::Validation(
                "Reset token cannot be empty".to_string(),
            ));
        }

        if new_password.is_empty() {
            return Err(AppError::Validation(
                "New password cannot be empty".to_string(),
            ));
        }

        if new_password.len() < crate::core::constants::MIN_PASSWORD_LENGTH {
            return Err(AppError::Validation(format!(
                "Password must be at least {} characters long",
                crate::core::constants::MIN_PASSWORD_LENGTH
            )));
        }

        Ok(Self {
            token,
            new_password,
        })
    }
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub keycloak_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn from_token_response(
        user_id: String,
        keycloak_id: String,
        token_response: TokenResponse,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(token_response.expires_in);

        Self {
            user_id,
            keycloak_id,
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn time_until_expiration(&self) -> Option<Duration> {
        let now = Utc::now();
        if now < self.expires_at {
            Some(self.expires_at - now)
        } else {
            None
        }
    }
}
