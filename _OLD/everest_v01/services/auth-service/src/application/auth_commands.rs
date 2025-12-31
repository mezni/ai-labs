// src/application/auth_commands.rs
use crate::application::auth_dtos::*;
use crate::core::{AppError, IdGenerator, errors::AppResult};
use crate::domain::{
    User, UserRegistration,
    repositories::{AuditLogRepository, RegistrationRepository, UserRepository},
    value_objects::*,
};
use crate::infrastructure::{CreateUserRequest, KeycloakClient, TokenBlacklist, UserCredential};
use std::sync::Arc;

pub struct AuthCommands {
    user_repo: Arc<dyn UserRepository>,
    registration_repo: Arc<dyn RegistrationRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
    keycloak_client: Arc<KeycloakClient>,
    token_blacklist: Arc<TokenBlacklist>,
}

impl AuthCommands {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        registration_repo: Arc<dyn RegistrationRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
        keycloak_client: Arc<KeycloakClient>,
        token_blacklist: Arc<TokenBlacklist>,
    ) -> Self {
        Self {
            user_repo,
            registration_repo,
            audit_repo,
            keycloak_client,
            token_blacklist,
        }
    }

    /// Register a new external user
    pub async fn register(
        &self,
        request: RegisterRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<RegisterResponse> {
        // Check if email already exists
        if self.user_repo.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Check if username already exists
        if self.user_repo.username_exists(&request.username).await? {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        // Check for pending registration
        if let Some(existing) = self
            .registration_repo
            .find_pending_by_email(&request.email)
            .await?
        {
            if !existing.is_expired() {
                return Err(AppError::Conflict(
                    "A pending registration already exists for this email".to_string(),
                ));
            }
        }

        // Parse domain objects
        let email = Email::new(request.email.clone())?;
        let username = Username::new(request.username.clone())?;
        let name = PersonName::new(request.first_name, request.last_name)?;
        let phone = PhoneNumber::new(request.phone)?;

        // Create registration
        let registration =
            UserRegistration::new(email, username, name, phone, ip_address, user_agent);

        // Save registration
        self.registration_repo.save(&registration).await?;

        // TODO: Send verification email with token/code

        Ok(RegisterResponse {
            registration_id: registration.registration_id.clone(),
            email: registration.email.as_str().to_string(),
            message: "Registration successful. Please verify your email.".to_string(),
            expires_at: registration.expires_at,
        })
    }

    /// Verify email and complete registration
    pub async fn verify_email(
        &self,
        request: VerifyEmailRequest,
    ) -> AppResult<VerifyEmailResponse> {
        // Find registration by token
        let mut registration = self
            .registration_repo
            .find_by_token(&request.token)
            .await?
            .ok_or_else(|| AppError::NotFound("Invalid verification token".to_string()))?;

        // Validate token
        if !registration.validate_token(&request.token) {
            return Err(AppError::BadRequest(
                "Token is invalid or expired".to_string(),
            ));
        }

        // Create user in Keycloak (password should be set during registration)
        let keycloak_request = CreateUserRequest {
            username: registration.username.as_str().to_string(),
            email: registration.email.as_str().to_string(),
            email_verified: true,
            enabled: true,
            first_name: registration.name.first_name().map(|s| s.to_string()),
            last_name: registration.name.last_name().map(|s| s.to_string()),
            credentials: None, // Password already set during registration
            attributes: None,
        };

        let keycloak_id = self.keycloak_client.create_user(keycloak_request).await?;

        // Create user in database
        let user = User::new_external(
            keycloak_id.clone(),
            registration.email.clone(),
            registration.username.clone(),
            registration.name.clone(),
            registration.phone.clone(),
        );

        self.user_repo.save(&user).await?;

        // Mark registration as verified
        registration.verify(keycloak_id, user.user_id.clone())?;
        self.registration_repo.update(&registration).await?;

        // Audit log
        self.audit_repo
            .log_email_verification(&user.user_id, user.email.as_str())
            .await?;

        Ok(VerifyEmailResponse {
            message: "Email verified successfully".to_string(),
            user_id: user.user_id,
            email: user.email.into(),
        })
    }

    /// Login user
    pub async fn login(
        &self,
        request: LoginRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<LoginResponse> {
        // Authenticate with Keycloak
        let token_response = self
            .keycloak_client
            .login(&request.username, &request.password)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid username or password".to_string()))?;

        // Find user by username or email
        let user = self
            .user_repo
            .find_by_username(&request.username)
            .await?
            .or_else(|| {
                // Try email
                futures::executor::block_on(self.user_repo.find_by_email(&request.username))
                    .ok()
                    .flatten()
            })
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Forbidden("User account is disabled".to_string()));
        }

        // Audit log
        self.audit_repo
            .log_login(
                &user.user_id,
                &user.keycloak_id,
                ip_address,
                user_agent,
                true,
            )
            .await?;

        Ok(LoginResponse {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            refresh_token: token_response.refresh_token,
            user: UserInfo {
                user_id: user.user_id,
                email: user.email.into(),
                username: user.username.as_str().to_string(),
                role: user.role.into(),
                network_id: user.network_id,
                station_id: user.station_id,
            },
        })
    }

    /// Refresh access token
    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> AppResult<TokenResponse> {
        let token_response = self
            .keycloak_client
            .refresh_token(&request.refresh_token)
            .await?;

        Ok(TokenResponse {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            refresh_token: token_response.refresh_token,
        })
    }

    /// Change password (self-service)
    pub async fn change_password(
        &self,
        user_id: String,
        request: ChangePasswordRequest,
    ) -> AppResult<()> {
        let user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify old password by attempting login
        let login_result = self
            .keycloak_client
            .login(user.username.as_str(), &request.old_password)
            .await;

        if login_result.is_err() {
            return Err(AppError::Unauthorized("Invalid old password".to_string()));
        }

        // Set new password
        self.keycloak_client
            .set_password(&user.keycloak_id, &request.new_password, false)
            .await?;

        // Audit log
        self.audit_repo
            .log_password_change(&user_id, &user_id)
            .await?;

        Ok(())
    }

    /// Request password reset
    pub async fn request_password_reset(
        &self,
        request: RequestPasswordResetRequest,
        ip_address: Option<String>,
    ) -> AppResult<()> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Send password reset email via Keycloak
        self.keycloak_client
            .send_password_reset_email(&user.keycloak_id)
            .await?;

        // Audit log
        self.audit_repo
            .log_password_reset_request(&user.user_id, user.email.as_str(), ip_address)
            .await?;

        Ok(())
    }

    /// Logout user
    pub async fn logout(
        &self,
        user_id: String,
        access_token: String,
        refresh_token: Option<String>,
    ) -> AppResult<()> {
        // Add access token to blacklist
        self.token_blacklist.blacklist_token(access_token, 3600);

        // Add refresh token to blacklist if provided
        if let Some(rt) = refresh_token {
            self.token_blacklist.blacklist_token(rt, 7 * 24 * 3600); // 7 days
        }

        // Audit log
        self.audit_repo.log_logout(&user_id).await?;

        Ok(())
    }
}
