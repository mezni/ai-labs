use crate::application::dto::{
    ChangePasswordRequest, CreateUserRequest, LoginRequest, RegisterRequest, UpdateUserRequest,
};
use crate::domain::{TokenResponse, User, UserRepository, UserRole, UserSource};
use crate::infrastructure::{DomainError, KeycloakClient};
use crate::utils::generate_user_id;
use std::sync::Arc;

const NO_NETWORK_ID: &str = "X";
const NO_STATION_ID: &str = "X";

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    keycloak_client: Arc<KeycloakClient>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, keycloak_client: Arc<KeycloakClient>) -> Self {
        Self {
            user_repo,
            keycloak_client,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<User, DomainError> {
        // Check if user already exists
        if self
            .user_repo
            .find_by_email(&request.email)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists(
                "User with this email already exists".to_string(),
            ));
        }

        if self
            .user_repo
            .find_by_username(&request.username)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists(
                "User with this username already exists".to_string(),
            ));
        }

        // Create user in Keycloak with USER role
        let keycloak_id = self
            .keycloak_client
            .create_user(
                &request.username,
                &request.email,
                &request.password,
                request.first_name.as_deref(),
                request.last_name.as_deref(),
                UserRole::User,
                Some(NO_NETWORK_ID),
                Some(NO_STATION_ID),
            )
            .await?;

        // Generate user ID
        let user_id = generate_user_id();

        // Store user in our database with source=web
        let user = self
            .user_repo
            .create(
                &user_id,
                &keycloak_id,
                &request.email,
                &request.username,
                request.first_name.as_deref(),
                request.last_name.as_deref(),
                request.phone.as_deref(),
                None, // No photo on registration
                UserRole::User.as_str(),
                NO_NETWORK_ID,
                NO_STATION_ID,
                UserSource::Web.as_str(),
                None, // No created_by for self-registration
            )
            .await?;

        Ok(user)
    }

    pub async fn create_user_by_admin(
        &self,
        request: CreateUserRequest,
        created_by: &str,
    ) -> Result<User, DomainError> {
        // Check if user already exists
        if self
            .user_repo
            .find_by_email(&request.email)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists(
                "User with this email already exists".to_string(),
            ));
        }

        if self
            .user_repo
            .find_by_username(&request.username)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists(
                "User with this username already exists".to_string(),
            ));
        }

        // Create user in Keycloak with specified role
        let keycloak_id = self
            .keycloak_client
            .create_user(
                &request.username,
                &request.email,
                &request.password,
                request.first_name.as_deref(),
                request.last_name.as_deref(),
                request.role,
                request.network_id.as_deref(),
                request.station_id.as_deref(),
            )
            .await?;

        // Generate user ID
        let user_id = generate_user_id();

        // Store user in our database with source=internal
        let user = self
            .user_repo
            .create(
                &user_id,
                &keycloak_id,
                &request.email,
                &request.username,
                request.first_name.as_deref(),
                request.last_name.as_deref(),
                request.phone.as_deref(),
                request.photo.as_deref(),
                request.role.as_str(),
                request.network_id.as_deref().unwrap_or(""),
                request.station_id.as_deref().unwrap_or(""),
                UserSource::Internal.as_str(),
                Some(created_by),
            )
            .await?;

        Ok(user)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<TokenResponse, DomainError> {
        // Authenticate with Keycloak
        let token = self
            .keycloak_client
            .login(&request.username, &request.password)
            .await?;

        Ok(TokenResponse {
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            refresh_token: token.refresh_token,
            refresh_expires_in: token.refresh_expires_in,
        })
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        request: ChangePasswordRequest,
    ) -> Result<(), DomainError> {
        // First verify old password by attempting login
        let user =
            self.user_repo.find_by_id(user_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("User with id {} not found", user_id))
            })?;

        // Verify old password
        self.keycloak_client
            .login(&user.username, &request.old_password)
            .await?;

        // Change password in Keycloak
        self.keycloak_client
            .change_password(&user.keycloak_id, &request.new_password)
            .await?;

        // Update timestamp in database
        self.user_repo.update_password_changed(user_id).await?;

        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, DomainError> {
        let token = self.keycloak_client.refresh_token(refresh_token).await?;

        Ok(TokenResponse {
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            refresh_token: token.refresh_token,
            refresh_expires_in: token.refresh_expires_in,
        })
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, DomainError> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User with id {} not found", user_id)))
    }

    pub async fn list_users(
        &self,
        role: Option<UserRole>,
        is_active: Option<bool>,
    ) -> Result<Vec<User>, DomainError> {
        self.user_repo.list_users(role, is_active).await
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        request: UpdateUserRequest,
        updated_by: &str,
    ) -> Result<User, DomainError> {
        self.user_repo
            .update_user(
                user_id,
                request.first_name.as_deref(),
                request.last_name.as_deref(),
                request.phone.as_deref(),
                request.photo.as_deref(),
                Some(updated_by),
            )
            .await
    }

    pub async fn deactivate_user(
        &self,
        user_id: &str,
        updated_by: &str,
    ) -> Result<(), DomainError> {
        self.user_repo
            .deactivate_user(user_id, Some(updated_by))
            .await
    }
}
