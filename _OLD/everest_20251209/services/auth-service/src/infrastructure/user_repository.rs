use crate::domain::{User, UserRepository, UserRole};
use crate::infrastructure::error::DomainError;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(
        &self,
        user_id: &str,
        keycloak_id: &str,
        email: &str,
        username: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        phone: Option<&str>,
        photo: Option<&str>,
        role: &str,
        network_id: &str,
        station_id: &str,
        source: &str,
        created_by: Option<&str>,
    ) -> Result<User, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                user_id, keycloak_id, email, username, first_name, last_name, 
                phone, photo, role, network_id, station_id, source, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            "#,
            user_id,
            keycloak_id,
            email,
            username,
            first_name,
            last_name,
            phone,
            photo,
            role,
            network_id,
            station_id,
            source,
            created_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            FROM users
            WHERE user_id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            FROM users
            WHERE keycloak_id = $1
            "#,
            keycloak_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn list_users(
        &self,
        role: Option<UserRole>,
        is_active: Option<bool>,
    ) -> Result<Vec<User>, DomainError> {
        let role_str = role.map(|r| r.as_str().to_string());

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            FROM users
            WHERE ($1::VARCHAR IS NULL OR role = $1)
              AND ($2::BOOLEAN IS NULL OR is_active = $2)
            ORDER BY created_at DESC
            "#,
            role_str,
            is_active
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn update_user(
        &self,
        user_id: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        phone: Option<&str>,
        photo: Option<&str>,
        updated_by: Option<&str>,
    ) -> Result<User, DomainError> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET 
                first_name = COALESCE($2, first_name),
                last_name = COALESCE($3, last_name),
                phone = COALESCE($4, phone),
                photo = COALESCE($5, photo),
                updated_by = $6,
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING 
                user_id as "user_id!",
                keycloak_id as "keycloak_id!",
                email as "email!",
                username as "username!",
                first_name,
                last_name,
                phone,
                photo,
                is_verified as "is_verified!",
                role as "role!",
                network_id as "network_id!",
                station_id as "station_id!",
                source as "source!",
                is_active as "is_active!",
                created_at as "created_at!",
                updated_at as "updated_at!",
                created_by,
                updated_by
            "#,
            user_id,
            first_name,
            last_name,
            phone,
            photo,
            updated_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_password_changed(&self, user_id: &str) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET updated_at = NOW()
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn deactivate_user(
        &self,
        user_id: &str,
        updated_by: Option<&str>,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET is_active = false, updated_by = $2, updated_at = NOW()
            WHERE user_id = $1
            "#,
            user_id,
            updated_by
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn verify_user(&self, user_id: &str) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET is_verified = true, updated_at = NOW()
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
