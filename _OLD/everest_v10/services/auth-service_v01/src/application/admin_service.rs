use crate::AppState;
use crate::core::errors::{AppError, AppResult};
use crate::core::utils::Generator;
use crate::domain::entities::User;
use crate::domain::enums::{Source, UserRole};
use crate::domain::repositories::UserRepository;
use crate::domain::services::{AdminService as AdminServiceTrait, CreateUserData, UpdateUserData};
use crate::infrastructure::keycloak_client::KeycloakClient;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct AdminService {
    // Keep the Arc<AppState> to access DB pools, repos, and clients
    state: Arc<AppState>,
}

impl AdminService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl AdminServiceTrait for AdminService {
async fn list_users(&self, limit: i64, offset: i64) -> AppResult<(Vec<User>, i64)> {
        let users = self.state.user_repo.list_active(limit, offset).await?;
        
        // Ensure your repository has this method (we added it in a previous step)
        let total = self.state.user_repo.count_active(None).await?;

        Ok((users, total))
    }

    async fn get_user(&self, user_id: &str) -> AppResult<User> {
        self.state
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    async fn create_user(&self, data: CreateUserData) -> AppResult<User> {
        // 1. Check DB for existing email
        if self
            .state
            .user_repo
            .find_by_email(&data.email)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict("Email already exists".into()));
        }

        // 2. Map role string to Enum
        let role = match data.role.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "partner" => UserRole::Partner,
            "operator" => UserRole::Operator,
            _ => UserRole::User,
        };

        // 3. Create in Keycloak (Admin operation)
        let keycloak_id = self
            .state
            .keycloak_client
            .create_user(&data.email, &data.username, &data.password, None)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        // 4. Assign role in Keycloak if not default
        if role != UserRole::User {
            self.state
                .keycloak_client
                .assign_role(&keycloak_id, &data.role)
                .await
                .map_err(|e| AppError::Keycloak(e.to_string()))?;
        }

        // 5. Build and save the User entity
        let user = User {
            user_id: Generator::generate_user_id(),
            keycloak_id,
            email: data.email,
            username: data.username,
            first_name: data.first_name,
            last_name: data.last_name,
            phone: data.phone,
            photo: None,
            is_verified: true,
            role,
            network_id: String::new(),
            station_id: String::new(),
            source: Source::Internal,
            is_active: true,
            deleted_at: None,
            last_login_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        };

        self.state.user_repo.create(&user).await
    }

    async fn update_user(&self, user_id: &str, data: UpdateUserData) -> AppResult<User> {
        let mut user = self
            .state
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if let Some(email) = data.email {
            user.email = email;
        }
        if let Some(username) = data.username {
            user.username = username;
        }
        if let Some(first_name) = data.first_name {
            user.first_name = Some(first_name);
        }
        if let Some(last_name) = data.last_name {
            user.last_name = Some(last_name);
        }
        if let Some(phone) = data.phone {
            user.phone = Some(phone);
        }

        if let Some(role_str) = data.role {
            user.role = match role_str.to_lowercase().as_str() {
                "admin" => UserRole::Admin,
                "partner" => UserRole::Partner,
                "operator" => UserRole::Operator,
                _ => UserRole::User,
            };
            // Note: In a real app, you'd likely want to update Keycloak roles here too
        }

        if let Some(is_active) = data.is_active {
            user.is_active = is_active;
            if is_active {
                self.state
                    .keycloak_client
                    .enable_user(&user.keycloak_id)
                    .await?;
            } else {
                self.state
                    .keycloak_client
                    .disable_user(&user.keycloak_id)
                    .await?;
            }
        }

        user.updated_at = Utc::now();
        self.state.user_repo.update(&user).await
    }

    async fn delete_user(&self, user_id: &str) -> AppResult<()> {
        let mut user = self
            .state
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        user.is_active = false;
        user.deleted_at = Some(Utc::now());

        // Ensure user cannot log in via Keycloak anymore
        self.state
            .keycloak_client
            .disable_user(&user.keycloak_id)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        self.state.user_repo.update(&user).await?;
        Ok(())
    }
}
