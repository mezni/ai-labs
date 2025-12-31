use crate::{
    application::dtos::*,
    core::{errors::AppError, id_generator},
    domain::{
        entities::{AuditLog, Role, User, UserSource},
        repositories::{AuditRepository, UserRepository},
    },
    infrastructure::keycloak::KeycloakClient,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{info, warn};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditRepository>,
    keycloak: Arc<KeycloakClient>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditRepository>,
        keycloak: Arc<KeycloakClient>,
    ) -> Self {
        Self {
            user_repo,
            audit_repo,
            keycloak,
        }
    }

    pub async fn register_external_user(
        &self,
        req: RegisterRequest,
        ip: Option<String>,
    ) -> Result<RegisterResponse, AppError> {
        if self.user_repo.find_user_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        let keycloak_user = self.keycloak.create_user(&req.email, &req.password).await?;
        let user_id = id_generator::generate_user_id();

        let user = User {
            id: user_id.clone(),
            keycloak_id: keycloak_user.id.clone(),
            email: req.email.clone(),
            source: UserSource::Web,
            roles: vec![Role::User],
            network_id: None,
            station_id: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let created_user = self.user_repo.create_user(&user).await?;

        self.keycloak
            .set_user_attributes(&keycloak_user.id, &user_id, &["user"])
            .await?;

        self.audit_repo
            .create_audit_log(&AuditLog {
                id: id_generator::generate_audit_id(),
                user_id: None,
                action: "user.register".to_string(),
                resource_type: "user".to_string(),
                resource_id: Some(user_id.clone()),
                details: Some(serde_json::json!({"email": req.email})),
                ip_address: ip,
                created_at: Utc::now(),
            })
            .await?;

        info!("External user registered: {}", user_id);

        Ok(RegisterResponse {
            user_id,
            email: created_user.email,
            message: "User registered successfully. Please verify your email.".to_string(),
        })
    }

    pub async fn create_internal_user(
        &self,
        req: CreateInternalUserRequest,
        admin_id: &str,
        ip: Option<String>,
    ) -> Result<UserResponse, AppError> {
        if self.user_repo.find_user_by_email(&req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        let roles: Vec<Role> = req
            .roles
            .iter()
            .filter_map(|r| Role::from_str(r))
            .collect();

        if roles.is_empty() {
            return Err(AppError::BadRequest("At least one valid role required".to_string()));
        }

        for role in &roles {
            match role {
                Role::Partner if req.network_id.is_none() => {
                    return Err(AppError::BadRequest("Partner role requires network_id".to_string()));
                }
                Role::Operator if req.network_id.is_none() || req.station_id.is_none() => {
                    return Err(AppError::BadRequest(
                        "Operator role requires network_id and station_id".to_string(),
                    ));
                }
                _ => {}
            }
        }

        let temp_password = format!("Temp{}!", nanoid::nanoid!(12));
        let keycloak_user = self.keycloak.create_user(&req.email, &temp_password).await?;
        let user_id = id_generator::generate_user_id();

        let user = User {
            id: user_id.clone(),
            keycloak_id: keycloak_user.id.clone(),
            email: req.email.clone(),
            source: UserSource::Internal,
            roles: roles.clone(),
            network_id: req.network_id.clone(),
            station_id: req.station_id.clone(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        user.validate_internal_user()
            .map_err(|e| AppError::BadRequest(e))?;

        let created_user = self.user_repo.create_user(&user).await?;

        let role_strs: Vec<&str> = roles.iter().map(|r| r.as_str()).collect();
        self.keycloak
            .set_user_attributes(&keycloak_user.id, &user_id, &role_strs)
            .await?;

        self.audit_repo
            .create_audit_log(&AuditLog {
                id: id_generator::generate_audit_id(),
                user_id: Some(admin_id.to_string()),
                action: "user.create_internal".to_string(),
                resource_type: "user".to_string(),
                resource_id: Some(user_id.clone()),
                details: Some(serde_json::json!({
                    "email": req.email,
                    "roles": role_strs,
                })),
                ip_address: ip,
                created_at: Utc::now(),
            })
            .await?;

        info!("Internal user created by {}: {}", admin_id, user_id);

        Ok(created_user.into())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<UserResponse, AppError> {
        let user = self
            .user_repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.is_deleted() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(user.into())
    }

    pub async fn list_users(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<ListUsersResponse, AppError> {
        let users = self.user_repo.list_users(limit, offset, search).await?;
        let total = users.len();

        Ok(ListUsersResponse {
            users: users.into_iter().map(|u| u.into()).collect(),
            total,
            limit,
            offset,
        })
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        req: UpdateUserRequest,
        admin_id: &str,
        ip: Option<String>,
    ) -> Result<UserResponse, AppError> {
        let mut user = self
            .user_repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.is_deleted() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        if let Some(roles) = req.roles {
            user.roles = roles.iter().filter_map(|r| Role::from_str(r)).collect();
        }

        if let Some(network_id) = req.network_id {
            user.network_id = Some(network_id);
        }

        if let Some(station_id) = req.station_id {
            user.station_id = Some(station_id);
        }

        if let Some(is_active) = req.is_active {
            user.is_active = is_active;
        }

        user.updated_at = Utc::now();

        if user.source == UserSource::Internal {
            user.validate_internal_user()
                .map_err(|e| AppError::BadRequest(e))?;
        }

        let updated_user = self.user_repo.update_user(&user).await?;

        let role_strs: Vec<&str> = user.roles.iter().map(|r| r.as_str()).collect();
        self.keycloak
            .set_user_attributes(&user.keycloak_id, &user.id, &role_strs)
            .await?;

        self.audit_repo
            .create_audit_log(&AuditLog {
                id: id_generator::generate_audit_id(),
                user_id: Some(admin_id.to_string()),
                action: "user.update".to_string(),
                resource_type: "user".to_string(),
                resource_id: Some(user_id.to_string()),
                details: None,
                ip_address: ip,
                created_at: Utc::now(),
            })
            .await?;

        info!("User {} updated by {}", user_id, admin_id);

        Ok(updated_user.into())
    }

    pub async fn update_me(
        &self,
        user_id: &str,
        req: UpdateMeRequest,
    ) -> Result<UserResponse, AppError> {
        let mut user = self
            .user_repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if let Some(email) = req.email {
            user.email = email;
        }

        user.updated_at = Utc::now();
        let updated_user = self.user_repo.update_user(&user).await?;

        Ok(updated_user.into())
    }

    pub async fn delete_user(
        &self,
        user_id: &str,
        admin_id: &str,
        ip: Option<String>,
    ) -> Result<(), AppError> {
        let user = self
            .user_repo
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.is_deleted() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        self.user_repo.soft_delete_user(user_id).await?;

        self.audit_repo
            .create_audit_log(&AuditLog {
                id: id_generator::generate_audit_id(),
                user_id: Some(admin_id.to_string()),
                action: "user.delete".to_string(),
                resource_type: "user".to_string(),
                resource_id: Some(user_id.to_string()),
                details: None,
                ip_address: ip,
                created_at: Utc::now(),
            })
            .await?;

        warn!("User {} soft deleted by {}", user_id, admin_id);

        Ok(())
    }
}