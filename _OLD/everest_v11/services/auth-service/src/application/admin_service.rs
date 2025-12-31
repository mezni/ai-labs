use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::admin::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::core::{
    constants::*,
    errors::{AppError, AppResult},
};
use crate::domain::{entities::User, repositories::UserRepository};
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
    keycloak_client: Arc<dyn KeycloakClient>,
}

impl AdminService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        keycloak_client: Arc<dyn KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            keycloak_client,
        }
    }

    pub async fn list_users(&self) -> AppResult<Vec<UserResponse>> {
        tracing::info!("Listing all users");

        let users = self.user_repo.find_all().await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_user(&self, id: Uuid) -> AppResult<UserResponse> {
        tracing::info!("Getting user: {}", id);

        let user = self.user_repo.find_by_id(&id).await?;

        Ok(user.into())
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> AppResult<UserResponse> {
        tracing::info!("Creating user: {}", req.email);

        // Check if user already exists
        if let Ok(_) = self.user_repo.find_by_email(&req.email).await {
            return Err(AppError::Conflict("User already exists".into()));
        }

        // Create user in Keycloak
        let keycloak_id = self
            .keycloak_client
            .create_user(&req.email, &req.username, &req.password, None)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        // Assign role if provided
        let role = req.role.as_deref().unwrap_or(ROLE_USER);
        if role == ROLE_ADMIN {
            self.keycloak_client
                .assign_role(&keycloak_id, ROLE_ADMIN)
                .await
                .map_err(|e| AppError::KeycloakError(e.to_string()))?;
        }

        // Create user in database
        let user = self
            .user_repo
            .create(&keycloak_id, &req.email, &req.username, role)
            .await?;

        tracing::info!("User created successfully: {}", user.id);

        Ok(user.into())
    }

    pub async fn update_user(&self, id: Uuid, req: UpdateUserRequest) -> AppResult<UserResponse> {
        tracing::info!("Updating user: {}", id);

        let mut user = self.user_repo.find_by_id(&id).await?;

        // Update fields if provided
        if let Some(email) = req.email {
            user.email = email;
        }
        if let Some(username) = req.username {
            user.username = username;
        }
        if let Some(status) = req.status {
            user.status = status;
        }
        if let Some(role) = req.role {
            // Update role in Keycloak
            self.keycloak_client
                .assign_role(&user.keycloak_id, &role)
                .await
                .map_err(|e| AppError::KeycloakError(e.to_string()))?;
            user.role = role;
        }

        user.updated_at = chrono::Utc::now();

        let updated_user = self.user_repo.update(&user).await?;

        tracing::info!("User updated successfully: {}", id);

        Ok(updated_user.into())
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        tracing::info!("Deleting user: {}", id);

        let user = self.user_repo.find_by_id(&id).await?;

        // Soft delete in database
        self.user_repo.soft_delete(&id).await?;

        // Disable user in Keycloak (we don't actually delete from Keycloak)
        self.keycloak_client
            .disable_user(&user.keycloak_id)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        tracing::info!("User deleted successfully: {}", id);

        Ok(())
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            keycloak_id: user.keycloak_id,
            email: user.email,
            username: user.username,
            status: user.status,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}