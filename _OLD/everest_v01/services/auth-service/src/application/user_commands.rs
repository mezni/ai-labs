// src/application/user_commands.rs
use crate::application::user_dtos::*;
use crate::core::{AppError, errors::AppResult};
use crate::domain::{
    User,
    repositories::{AuditLogRepository, UserRepository},
    value_objects::*,
};
use crate::infrastructure::{CreateUserRequest, KeycloakClient, UserCredential};
use chrono::Utc;
use std::sync::Arc;

pub struct UserCommands {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
    keycloak_client: Arc<KeycloakClient>,
}

impl UserCommands {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
        keycloak_client: Arc<KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            audit_repo,
            keycloak_client,
        }
    }

    /// Create internal user (admin-only)
    pub async fn create_internal_user(
        &self,
        request: CreateInternalUserRequest,
        created_by: String,
    ) -> AppResult<UserResponse> {
        // Validate email uniqueness
        if self.user_repo.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        // Validate username uniqueness
        if self.user_repo.username_exists(&request.username).await? {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Parse and validate domain objects
        let email = Email::new(request.email.clone())?;
        let username = Username::new(request.username.clone())?;
        let role = UserRole::new(request.role.clone())?;
        let name = PersonName::new(request.first_name.clone(), request.last_name.clone())?;
        let phone = PhoneNumber::new(request.phone.clone())?;

        let network_id = request
            .network_id
            .as_ref()
            .map(|id| NetworkId::new(id.clone()))
            .transpose()?;

        let station_id = request
            .station_id
            .as_ref()
            .map(|id| StationId::new(id.clone()))
            .transpose()?;

        // Create user in Keycloak first
        let keycloak_request = CreateUserRequest {
            username: username.as_str().to_string(),
            email: email.as_str().to_string(),
            email_verified: true,
            enabled: true,
            first_name: request.first_name,
            last_name: request.last_name,
            credentials: Some(vec![UserCredential {
                credential_type: "password".to_string(),
                value: request.password,
                temporary: false,
            }]),
            attributes: None,
        };

        let keycloak_id = self
            .keycloak_client
            .create_user(keycloak_request)
            .await
            .map_err(|e| AppError::Keycloak(format!("Failed to create user in Keycloak: {}", e)))?;

        // Assign role in Keycloak
        if let Err(e) = self
            .keycloak_client
            .assign_role(&keycloak_id, role.as_str())
            .await
        {
            tracing::warn!(
                "Failed to assign role in Keycloak for user {}: {}",
                keycloak_id,
                e
            );
        }

        // Create user in database
        let user = User::new_internal(
            keycloak_id,
            email,
            username,
            name,
            phone,
            role,
            network_id,
            station_id,
            created_by.clone(),
        )?;

        self.user_repo.save(&user).await?;

        // Audit log
        self.audit_repo
            .log_user_creation(
                &user.user_id,
                Some(&created_by),
                &format!("Internal user created with role: {}", user.role.as_str()),
            )
            .await?;

        Ok(UserResponse::from(user))
    }

    /// Update user profile (self-update)
    pub async fn update_profile(
        &self,
        user_id: String,
        request: UpdateProfileRequest,
    ) -> AppResult<UserResponse> {
        let mut user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let name = PersonName::new(request.first_name, request.last_name)?;
        let phone = PhoneNumber::new(request.phone)?;

        user.update_profile(name, phone, request.photo)?;
        self.user_repo.update(&user).await?;

        // Audit log
        self.audit_repo
            .log_user_update(&user_id, &user_id, "Profile updated")
            .await?;

        Ok(UserResponse::from(user))
    }

    /// Admin update user
    pub async fn admin_update_user(
        &self,
        user_id: String,
        request: AdminUpdateUserRequest,
        updated_by: String,
    ) -> AppResult<UserResponse> {
        let mut user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let old_role = user.role.as_str().to_string();
        let old_email = user.email.as_str().to_string();

        // Parse optional fields
        let email = request
            .email
            .as_ref()
            .map(|e| Email::new(e.clone()))
            .transpose()?;

        let username = request
            .username
            .as_ref()
            .map(|u| Username::new(u.clone()))
            .transpose()?;

        let name = if request.first_name.is_some() || request.last_name.is_some() {
            Some(PersonName::new(
                request.first_name.clone(),
                request.last_name.clone(),
            )?)
        } else {
            None
        };

        let phone = if request.phone.is_some() {
            Some(PhoneNumber::new(request.phone.clone())?)
        } else {
            None
        };

        let role = UserRole::new(request.role.clone())?;

        let network_id = request
            .network_id
            .as_ref()
            .map(|id| NetworkId::new(id.clone()))
            .transpose()?;

        let station_id = request
            .station_id
            .as_ref()
            .map(|id| StationId::new(id.clone()))
            .transpose()?;

        // Update user
        user.admin_update(
            email.clone(),
            username.clone(),
            name,
            phone,
            request.photo.clone(),
            Some(role.clone()),
            network_id,
            station_id,
            updated_by.clone(),
        )?;

        self.user_repo.update(&user).await?;

        // Sync with Keycloak
        // Sync with Keycloak
        if let Err(e) = self
            .keycloak_client
            .update_user(
                &user.keycloak_id,
                email.as_ref().map(|e| e.clone().into()), // Use as_ref() to borrow
                username.as_ref().map(|u| u.as_str().to_string()), // Use as_ref() to borrow
                request.first_name,
                request.last_name,
                Some(user.is_active),
            )
            .await
        {
            tracing::warn!("Failed to update user in Keycloak: {}", e);
        }

        // Update role in Keycloak if changed
        if role.as_str() != old_role {
            if let Err(e) = self
                .keycloak_client
                .assign_role(&user.keycloak_id, role.as_str())
                .await
            {
                tracing::warn!("Failed to update role in Keycloak: {}", e);
            }

            self.audit_repo
                .log_role_change(&user_id, &old_role, role.as_str(), &updated_by)
                .await?;
        }

        // Audit log
        let mut changes = Vec::new();
        if email.is_some() && user.email.as_str() != old_email {
            changes.push("email".to_string());
        }
        if username.is_some() {
            changes.push("username".to_string());
        }
        if role.as_str() != old_role {
            changes.push("role".to_string());
        }

        self.audit_repo
            .log_user_update(
                &user_id,
                &updated_by,
                &format!("Updated: {}", changes.join(", ")),
            )
            .await?;

        Ok(UserResponse::from(user))
    }

    /// Soft delete user
    pub async fn delete_user(
        &self,
        user_id: String,
        deleted_by: String,
    ) -> AppResult<DeleteUserResponse> {
        let mut user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.soft_delete(deleted_by.clone())?;
        self.user_repo.update(&user).await?;

        // Disable in Keycloak
        if let Err(e) = self.keycloak_client.disable_user(&user.keycloak_id).await {
            tracing::warn!("Failed to disable user in Keycloak: {}", e);
        }

        // Audit log
        self.audit_repo
            .log_user_deletion(&user_id, &deleted_by)
            .await?;

        Ok(DeleteUserResponse {
            message: "User deleted successfully".to_string(),
            user_id,
            deleted_at: user.deleted_at.unwrap(),
        })
    }

    /// Admin reset password
    pub async fn admin_reset_password(
        &self,
        user_id: String,
        request: AdminResetPasswordRequest,
        admin_id: String,
    ) -> AppResult<()> {
        let user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Set password in Keycloak
        self.keycloak_client
            .set_password(&user.keycloak_id, &request.new_password, request.temporary)
            .await?;

        // Audit log
        self.audit_repo
            .log_password_change(&user_id, &admin_id)
            .await?;

        Ok(())
    }
}
