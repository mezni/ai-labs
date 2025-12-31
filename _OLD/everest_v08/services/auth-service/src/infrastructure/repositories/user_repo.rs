use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, error};

use crate::core::errors::AppError;
use crate::domain::{
    entities::User,
    repositories::UserRepository as UserRepositoryTrait,
};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        debug!("Creating user with ID: {}", user.user_id);
        
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                user_id, keycloak_id, email, username,
                first_name, last_name, phone, photo,
                is_verified, role, network_id, station_id, source,
                is_active, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(&user.source)
        .bind(user.is_active)
        .bind(&user.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create user: {}", e);
            match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                    AppError::Conflict("User with this email or username already exists".to_string())
                }
                _ => AppError::DatabaseError(e.to_string()),
            }
        })?;

        debug!("User created successfully: {}", user.user_id);
        Ok(user)
    }

    async fn get_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        debug!("Fetching user by ID: {}", user_id);
        
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE user_id = $1 AND deleted_at IS NULL"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user by ID: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        debug!("Fetching user by email: {}", email);
        
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user by email: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        debug!("Fetching user by username: {}", username);
        
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 AND deleted_at IS NULL"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user by username: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    async fn get_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, AppError> {
        debug!("Fetching user by Keycloak ID: {}", keycloak_id);
        
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE keycloak_id = $1 AND deleted_at IS NULL"
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user by Keycloak ID: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<User>, AppError> {
        debug!("Fetching all users");
        
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE deleted_at IS NULL ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch all users: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        debug!("Found {} users", users.len());
        Ok(users)
    }

    async fn get_active_users(&self) -> Result<Vec<User>, AppError> {
        debug!("Fetching active users");
        
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE is_active = true AND deleted_at IS NULL ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch active users: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        debug!("Found {} active users", users.len());
        Ok(users)
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        debug!("Updating user: {}", user.user_id);
        
        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET 
                email = $2,
                username = $3,
                first_name = $4,
                last_name = $5,
                phone = $6,
                photo = $7,
                is_verified = $8,
                role = $9,
                network_id = $10,
                station_id = $11,
                is_active = $12,
                updated_by = $13,
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.phone)
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(&user.role)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(&user.updated_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update user: {}", e);
            match e {
                sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                    AppError::Conflict("Email or username already exists".to_string())
                }
                _ => AppError::DatabaseError(e.to_string()),
            }
        })?;

        debug!("User updated successfully: {}", user.user_id);
        Ok(updated_user)
    }

    async fn soft_delete(&self, user_id: &str) -> Result<(), AppError> {
        debug!("Soft deleting user: {}", user_id);
        
        let result = sqlx::query(
            r#"
            UPDATE users
            SET 
                is_active = false,
                deleted_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to soft delete user: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        debug!("User soft deleted successfully: {}", user_id);
        Ok(())
    }

    async fn update_last_login(&self, user_id: &str) -> Result<(), AppError> {
        debug!("Updating last login for user: {}", user_id);
        
        sqlx::query(
            "UPDATE users SET last_login_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE user_id = $1"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update last login: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        debug!("Checking if email exists: {}", email);
        
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND deleted_at IS NULL)"
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to check email existence: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(exists)
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        debug!("Checking if username exists: {}", username);
        
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND deleted_at IS NULL)"
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to check username existence: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(exists)
    }

    async fn count(&self) -> Result<i64, AppError> {
        debug!("Counting total users");
        
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE deleted_at IS NULL"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to count users: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;

        Ok(count)
    }
}