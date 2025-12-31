use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::entities::{User, Registration, Invitation};
use crate::domain::enums::{Role, Source};
use crate::domain::value_objects::{Email, Username, Password, PhoneNumber};
use crate::core::errors::AppError;

/// Domain service for password operations
pub trait PasswordService: Send + Sync {
    /// Hash a plain text password
    fn hash_password(&self, password: &Password) -> Result<String, AppError>;
    
    /// Verify a password against a hash
    fn verify_password(&self, password: &Password, hash: &str) -> Result<bool, AppError>;
    
    /// Generate a random password
    fn generate_password(&self, length: usize) -> String;
}

/// Domain service for token generation
pub trait TokenService: Send + Sync {
    /// Generate a verification token
    fn generate_verification_token(&self) -> String;
    
    /// Generate a verification code (6 digits)
    fn generate_verification_code(&self) -> String;
    
    /// Generate an invitation token
    fn generate_invitation_token(&self) -> String;
    
    /// Generate JWT access token
    fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: Role,
    ) -> Result<String, AppError>;
    
    /// Generate JWT refresh token
    fn generate_refresh_token(&self) -> String;
    
    /// Validate and decode JWT token
    fn validate_token(&self, token: &str) -> Result<TokenClaims, AppError>;
}

/// Token claims structure
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub user_id: String,
    pub email: String,
    pub role: Role,
    pub exp: i64,
    pub iat: i64,
}

/// Domain service for email operations
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send verification email with token and code
    async fn send_verification_email(
        &self,
        email: &Email,
        token: &str,
        code: &str,
    ) -> Result<(), AppError>;
    
    /// Send invitation email
    async fn send_invitation_email(
        &self,
        email: &Email,
        token: &str,
        invited_by: &str,
    ) -> Result<(), AppError>;
    
    /// Send password reset email
    async fn send_password_reset_email(
        &self,
        email: &Email,
        token: &str,
    ) -> Result<(), AppError>;
    
    /// Send welcome email
    async fn send_welcome_email(
        &self,
        email: &Email,
        username: &Username,
    ) -> Result<(), AppError>;
}

/// Domain service for SMS operations
#[async_trait]
pub trait SmsService: Send + Sync {
    /// Send verification SMS with code
    async fn send_verification_sms(
        &self,
        phone: &PhoneNumber,
        code: &str,
    ) -> Result<(), AppError>;
    
    /// Send password reset SMS
    async fn send_password_reset_sms(
        &self,
        phone: &PhoneNumber,
        code: &str,
    ) -> Result<(), AppError>;
}

/// Domain service for Keycloak integration
#[async_trait]
pub trait KeycloakService: Send + Sync {
    /// Create a user in Keycloak
    async fn create_user(
        &self,
        email: &Email,
        username: &Username,
        password: &Password,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<String, AppError>;
    
    /// Update user in Keycloak
    async fn update_user(
        &self,
        keycloak_id: &str,
        email: Option<&Email>,
        username: Option<&Username>,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<(), AppError>;
    
    /// Delete user from Keycloak
    async fn delete_user(&self, keycloak_id: &str) -> Result<(), AppError>;
    
    /// Enable/disable user in Keycloak
    async fn set_user_enabled(&self, keycloak_id: &str, enabled: bool) -> Result<(), AppError>;
    
    /// Update user role in Keycloak
    async fn update_user_role(&self, keycloak_id: &str, role: Role) -> Result<(), AppError>;
    
    /// Validate user credentials with Keycloak
    async fn validate_credentials(
        &self,
        email_or_username: &str,
        password: &Password,
    ) -> Result<KeycloakUser, AppError>;
    
    /// Get user from Keycloak by ID
    async fn get_user(&self, keycloak_id: &str) -> Result<Option<KeycloakUser>, AppError>;
    
    /// Get user from Keycloak by email
    async fn get_user_by_email(&self, email: &Email) -> Result<Option<KeycloakUser>, AppError>;
}

/// Keycloak user representation
#[derive(Debug, Clone)]
pub struct KeycloakUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub enabled: bool,
    pub email_verified: bool,
}

/// Domain service for user validation
pub trait UserValidationService: Send + Sync {
    /// Validate user registration data
    fn validate_registration(
        &self,
        email: &str,
        username: &str,
        password: &str,
    ) -> Result<(Email, Username, Password), AppError>;
    
    /// Validate email format
    fn validate_email(&self, email: &str) -> Result<Email, AppError>;
    
    /// Validate username format
    fn validate_username(&self, username: &str) -> Result<Username, AppError>;
    
    /// Validate password strength
    fn validate_password(&self, password: &str) -> Result<Password, AppError>;
    
    /// Validate phone number format
    fn validate_phone(&self, phone: &str) -> Result<PhoneNumber, AppError>;
}

/// Domain service for rate limiting
#[async_trait]
pub trait RateLimitService: Send + Sync {
    /// Check if an action is rate limited
    async fn check_rate_limit(
        &self,
        identifier: &str,
        action: &str,
        max_attempts: i32,
        window_seconds: i64,
    ) -> Result<bool, AppError>;
    
    /// Record an action for rate limiting
    async fn record_action(
        &self,
        identifier: &str,
        action: &str,
    ) -> Result<(), AppError>;
    
    /// Reset rate limit for an identifier and action
    async fn reset_rate_limit(
        &self,
        identifier: &str,
        action: &str,
    ) -> Result<(), AppError>;
}

/// Domain service for audit logging
#[async_trait]
pub trait AuditService: Send + Sync {
    /// Log authentication attempt
    async fn log_auth_attempt(
        &self,
        user_id: Option<&str>,
        action: &str,
        success: bool,
        error: Option<&str>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), AppError>;
    
    /// Log Keycloak sync operation
    async fn log_keycloak_sync(
        &self,
        user_id: Option<&str>,
        keycloak_id: Option<&str>,
        action: &str,
        status: &str,
        details: Option<&str>,
        error: Option<&str>,
    ) -> Result<(), AppError>;
    
    /// Log email verification attempt
    async fn log_email_verification(
        &self,
        user_id: Option<&str>,
        email: &str,
        token: Option<&str>,
        verified: bool,
        ip_address: Option<&str>,
    ) -> Result<(), AppError>;
}

/// Domain service for ID generation
pub trait IdGeneratorService: Send + Sync {
    /// Generate a unique user ID
    fn generate_user_id(&self) -> String;
    
    /// Generate a unique registration ID
    fn generate_registration_id(&self) -> String;
    
    /// Generate a unique invitation ID
    fn generate_invitation_id(&self) -> String;
    
    /// Generate a unique token ID
    fn generate_token_id(&self) -> String;
}

/// Domain service for business rules
pub trait BusinessRulesService: Send + Sync {
    /// Check if user can be registered
    fn can_register(&self, email: &Email, username: &Username) -> Result<(), AppError>;
    
    /// Check if registration can be verified
    fn can_verify_registration(&self, registration: &Registration) -> Result<(), AppError>;
    
    /// Check if invitation can be accepted
    fn can_accept_invitation(&self, invitation: &Invitation) -> Result<(), AppError>;
    
    /// Check if user can login
    fn can_login(&self, user: &User) -> Result<(), AppError>;
    
    /// Get default token expiration times
    fn get_access_token_expiry(&self) -> i64;
    fn get_refresh_token_expiry(&self) -> i64;
    fn get_verification_expiry(&self) -> i64;
    fn get_invitation_expiry(&self) -> i64;
}