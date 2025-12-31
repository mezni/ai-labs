use crate::domain::entities::{User, UserRegistration};
use crate::domain::repositories::{RepositoryError, UserRegistrationRepository, UserRepository};
use crate::domain::value_objects::*;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use std::net::IpAddr;
use std::sync::Arc;

// ============================================================================
// Service Error Type
// ============================================================================

#[derive(Debug)]
pub enum ServiceError {
    ValidationError(String),
    NotFound(String),
    AlreadyExists(String),
    Unauthorized(String),
    BusinessRuleViolation(String),
    ExternalServiceError(String),
    RepositoryError(RepositoryError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::BusinessRuleViolation(msg) => write!(f, "Business rule violation: {}", msg),
            Self::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            Self::RepositoryError(err) => write!(f, "Repository error: {}", err),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        Self::RepositoryError(err)
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;

// ============================================================================
// Registration Service
// ============================================================================

pub struct RegistrationService {
    registration_repo: Arc<dyn UserRegistrationRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl RegistrationService {
    pub fn new(
        registration_repo: Arc<dyn UserRegistrationRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            registration_repo,
            user_repo,
        }
    }

    /// Create a new user registration
    pub async fn create_registration(
        &self,
        email: Email,
        username: Option<Username>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<Phone>,
        verification_token_hash: String,
        verification_method: VerificationMethod,
        context: RegistrationContext,
    ) -> ServiceResult<UserRegistration> {
        // Check if email already exists in pending registrations
        if let Some(existing) = self.registration_repo.find_by_email(&email).await? {
            if existing.status == RegistrationStatus::Pending {
                return Err(ServiceError::AlreadyExists(
                    "Active registration already exists for this email".to_string(),
                ));
            }
        }

        // Check if email already exists in users
        if self.user_repo.email_exists(&email).await? {
            return Err(ServiceError::AlreadyExists(
                "User with this email already exists".to_string(),
            ));
        }

        // Check username uniqueness if provided
        if let Some(ref username) = username {
            if self.user_repo.username_exists(username.value()).await? {
                return Err(ServiceError::AlreadyExists(
                    "Username already taken".to_string(),
                ));
            }
        }

        // Create registration
        let expires_at = Utc::now() + Duration::days(7);
        let mut registration = UserRegistration::new(
            email,
            verification_token_hash,
            verification_method,
            context,
            expires_at,
        );

        registration.username = username;
        registration.first_name = first_name;
        registration.last_name = last_name;
        registration.phone = phone;

        self.registration_repo.create(&registration).await?;

        Ok(registration)
    }

    /// Verify a registration
    pub async fn verify_registration(
        &self,
        token_hash: &str,
        ip: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> ServiceResult<UserRegistration> {
        let mut registration = self
            .registration_repo
            .find_by_token_hash(token_hash)
            .await?
            .ok_or_else(|| ServiceError::NotFound("Registration not found".to_string()))?;

        if !registration.can_verify() {
            return Err(ServiceError::BusinessRuleViolation(
                "Registration cannot be verified (expired or invalid status)".to_string(),
            ));
        }

        let context = VerificationContext {
            ip,
            user_agent,
            verified_at: Utc::now(),
        };

        registration.verify(context).map_err(ServiceError::ValidationError)?;
        self.registration_repo.update(&registration).await?;

        Ok(registration)
    }

    /// Cancel a registration
    pub async fn cancel_registration(
        &self,
        registration_id: &str,
        reason: Option<String>,
    ) -> ServiceResult<UserRegistration> {
        let mut registration = self.registration_repo.find_by_id(registration_id).await?;

        if registration.status != RegistrationStatus::Pending {
            return Err(ServiceError::BusinessRuleViolation(
                "Only pending registrations can be cancelled".to_string(),
            ));
        }

        registration.cancel(reason);
        self.registration_repo.update(&registration).await?;

        Ok(registration)
    }

    /// Get registration status
    pub async fn get_registration_status(
        &self,
        registration_id: &str,
    ) -> ServiceResult<UserRegistration> {
        self.registration_repo.find_by_id(registration_id).await.map_err(Into::into)
    }

    /// Resend verification
    pub async fn resend_verification(
        &self,
        email: &Email,
        new_token_hash: String,
    ) -> ServiceResult<UserRegistration> {
        let mut registration = self
            .registration_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| ServiceError::NotFound("Registration not found".to_string()))?;

        if registration.status != RegistrationStatus::Pending {
            return Err(ServiceError::BusinessRuleViolation(
                "Cannot resend verification for non-pending registration".to_string(),
            ));
        }

        registration.verification_token_hash = new_token_hash;
        registration.verification_token_expires_at = Utc::now() + Duration::hours(24);
        registration.updated_at = Utc::now();

        self.registration_repo.update(&registration).await?;

        Ok(registration)
    }

    /// Expire old registrations (background job)
    pub async fn expire_old_registrations(&self) -> ServiceResult<i64> {
        let expired = self
            .registration_repo
            .find_expired(Utc::now(), 100)
            .await?;

        let mut count = 0;
        for mut registration in expired {
            registration.mark_expired();
            self.registration_repo.update(&registration).await?;
            count += 1;
        }

        Ok(count)
    }

    /// Mark registration as Keycloak synced
    pub async fn mark_keycloak_synced(
        &self,
        registration_id: &str,
        keycloak_id: String,
    ) -> ServiceResult<()> {
        let mut registration = self.registration_repo.find_by_id(registration_id).await?;
        registration.mark_keycloak_synced(keycloak_id);
        self.registration_repo.update(&registration).await?;
        Ok(())
    }

    /// Mark registration as Keycloak failed
    pub async fn mark_keycloak_failed(
        &self,
        registration_id: &str,
        error: String,
    ) -> ServiceResult<()> {
        let mut registration = self.registration_repo.find_by_id(registration_id).await?;
        registration.mark_keycloak_failed(error);
        self.registration_repo.update(&registration).await?;
        Ok(())
    }
}

// ============================================================================
// User Service
// ============================================================================

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Create a new user from verified registration
    pub async fn create_from_registration(
        &self,
        registration: &UserRegistration,
        keycloak_id: String,
    ) -> ServiceResult<User> {
        if !registration.is_verified() {
            return Err(ServiceError::BusinessRuleViolation(
                "Registration must be verified".to_string(),
            ));
        }

        // Check if user already exists
        if self.user_repo.email_exists(&registration.email).await? {
            return Err(ServiceError::AlreadyExists(
                "User already exists".to_string(),
            ));
        }

        let mut user = User::new(
            keycloak_id,
            registration.email.clone(),
            Some(registration.registration_id.clone()),
            registration.context.source,
        );

        user.username = registration.username.clone();
        user.first_name = registration.first_name.clone();
        user.last_name = registration.last_name.clone();
        user.phone = registration.phone.clone();
        user.email_verified = true;
        user.registration_ip = registration.context.ip;
        user.registration_user_agent = registration.context.user_agent.clone();
        user.source_details = serde_json::json!({
            "campaign_id": registration.context.campaign_id,
            "utm_source": registration.context.utm_source,
            "utm_medium": registration.context.utm_medium,
            "utm_campaign": registration.context.utm_campaign,
        });

        self.user_repo.create(&user).await?;

        Ok(user)
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: &str) -> ServiceResult<User> {
        self.user_repo.find_by_id(user_id).await.map_err(Into::into)
    }

    /// Get user by Keycloak ID
    pub async fn get_user_by_keycloak_id(&self, keycloak_id: &str) -> ServiceResult<Option<User>> {
        self.user_repo.find_by_keycloak_id(keycloak_id).await.map_err(Into::into)
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &Email) -> ServiceResult<Option<User>> {
        self.user_repo.find_by_email(email).await.map_err(Into::into)
    }

    /// Activate user
    pub async fn activate_user(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.activate();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Suspend user
    pub async fn suspend_user(&self, user_id: &str, reason: Option<String>) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.suspend(reason);
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Lock user account
    pub async fn lock_user(&self, user_id: &str, duration: Duration) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.lock(duration);
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Unlock user account
    pub async fn unlock_user(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.unlock();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Record successful login
    pub async fn record_login(
        &self,
        user_id: &str,
        ip: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        if !user.is_active() {
            return Err(ServiceError::Unauthorized("User is not active".to_string()));
        }

        if user.is_locked() {
            return Err(ServiceError::Unauthorized("User account is locked".to_string()));
        }

        user.record_login(ip, user_agent);
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Record failed login attempt
    pub async fn record_failed_login(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.record_failed_login();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Update user activity
    pub async fn update_activity(&self, user_id: &str) -> ServiceResult<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.update_activity();
        self.user_repo.update(&user).await?;
        Ok(())
    }

    /// Verify user email
    pub async fn verify_email(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.verify_email();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Enable MFA
    pub async fn enable_mfa(&self, user_id: &str, method: MfaMethod) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.enable_mfa(method);
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Disable MFA
    pub async fn disable_mfa(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.disable_mfa();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Accept terms and conditions
    pub async fn accept_terms(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.accept_terms();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Accept privacy policy
    pub async fn accept_privacy(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.accept_privacy();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Soft delete user
    pub async fn delete_user(&self, user_id: &str) -> ServiceResult<User> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.soft_delete();
        self.user_repo.update(&user).await?;
        Ok(user)
    }

    /// Search users
    pub async fn search_users(
        &self,
        query: &str,
        offset: i64,
        limit: i32,
    ) -> ServiceResult<Vec<User>> {
        self.user_repo.search(query, offset, limit).await.map_err(Into::into)
    }

    /// List users with pagination
    pub async fn list_users(&self, offset: i64, limit: i32) -> ServiceResult<Vec<User>> {
        self.user_repo.list(offset, limit).await.map_err(Into::into)
    }

    /// Get user count
    pub async fn count_users(&self) -> ServiceResult<i64> {
        self.user_repo.count().await.map_err(Into::into)
    }
}

// ============================================================================
// Password Service (placeholder for password operations)
// ============================================================================

pub struct PasswordService {
    user_repo: Arc<dyn UserRepository>,
}

impl PasswordService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Request password reset
    pub async fn request_password_reset(&self, email: &Email) -> ServiceResult<()> {
        // TODO: Implement password reset logic
        // 1. Find user by email
        // 2. Generate reset token
        // 3. Store token hash
        // 4. Send email
        Ok(())
    }

    /// Confirm password reset
    pub async fn confirm_password_reset(
        &self,
        token: &str,
        new_password: &str,
    ) -> ServiceResult<()> {
        // TODO: Implement password reset confirmation
        // 1. Validate token
        // 2. Update password in Keycloak
        // 3. Invalidate token
        Ok(())
    }

    /// Change password
    pub async fn change_password(
        &self,
        user_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> ServiceResult<()> {
        // TODO: Implement password change
        // 1. Verify old password with Keycloak
        // 2. Validate new password policy
        // 3. Update password in Keycloak
        // 4. Record password change event
        Ok(())
    }

    /// Get password policy
    pub fn get_password_policy(&self) -> PasswordPolicy {
        PasswordPolicy {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: i32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
}