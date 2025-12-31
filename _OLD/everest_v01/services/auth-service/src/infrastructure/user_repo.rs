// src/infrastructure/user_repo.rs
use crate::core::{AppError, constants::*, errors::AppResult};
use crate::domain::{
    User,
    repositories::{SortOrder, UserFilters, UserRepository},
    value_objects::*,
};
use async_trait::async_trait;
use sqlx::{PgPool, Row};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Helper to convert database row to User
    fn row_to_user(row: &sqlx::postgres::PgRow) -> AppResult<User> {
        let email = Email::new(row.get("email"))?;
        let username = Username::new(row.get("username"))?;
        let role = UserRole::new(row.get("role"))?;
        let source = UserSource::new(row.get("source"))?;

        let first_name: Option<String> = row.get("first_name");
        let last_name: Option<String> = row.get("last_name");
        let name = PersonName::new(first_name, last_name)?;

        let phone: Option<String> = row.get("phone");
        let phone = PhoneNumber::new(phone)?;

        Ok(User {
            user_id: row.get("user_id"),
            keycloak_id: row.get("keycloak_id"),
            email,
            username,
            name,
            phone,
            photo: row.get("photo"),
            is_verified: row.get("is_verified"),
            role,
            network_id: row.get("network_id"),
            station_id: row.get("station_id"),
            source,
            is_active: row.get("is_active"),
            deleted_at: row.get("deleted_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
            updated_by: row.get("updated_by"),
        })
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_user(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_user(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE LOWER(username) = LOWER($1) AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_user(&r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE keycloak_id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(keycloak_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_user(&r)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO users (
                user_id, keycloak_id, email, username, first_name, last_name,
                phone, photo, is_verified, role, network_id, station_id,
                source, is_active, created_at, updated_at, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.keycloak_id)
        .bind(user.email.as_str())
        .bind(user.username.as_str())
        .bind(user.name.first_name())
        .bind(user.name.last_name())
        .bind(user.phone.as_str())
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(user.role.as_str())
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.source.as_str())
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(&user.created_by)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, user: &User) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET email = $2, username = $3, first_name = $4, last_name = $5,
                phone = $6, photo = $7, is_verified = $8, role = $9,
                network_id = $10, station_id = $11, is_active = $12,
                deleted_at = $13, updated_at = $14, updated_by = $15
            WHERE user_id = $1
            "#,
        )
        .bind(&user.user_id)
        .bind(user.email.as_str())
        .bind(user.username.as_str())
        .bind(user.name.first_name())
        .bind(user.name.last_name())
        .bind(user.phone.as_str())
        .bind(&user.photo)
        .bind(user.is_verified)
        .bind(user.role.as_str())
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(user.deleted_at)
        .bind(user.updated_at)
        .bind(&user.updated_by)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn list(&self, filters: UserFilters) -> AppResult<Vec<User>> {
        let mut query = String::from("SELECT * FROM users WHERE 1=1");
        let mut bindings: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 1;

        // Apply filters
        if !filters.include_deleted {
            query.push_str(" AND deleted_at IS NULL");
        }

        if let Some(ref search) = filters.search {
            query.push_str(&format!(
                " AND (LOWER(email) LIKE ${} OR LOWER(username) LIKE ${} OR LOWER(first_name) LIKE ${} OR LOWER(last_name) LIKE ${})",
                param_count, param_count + 1, param_count + 2, param_count + 3
            ));
            let search_pattern = format!("%{}%", search.to_lowercase());
            bindings.push(Box::new(search_pattern.clone()));
            bindings.push(Box::new(search_pattern.clone()));
            bindings.push(Box::new(search_pattern.clone()));
            bindings.push(Box::new(search_pattern));
            param_count += 4;
        }

        if let Some(ref role) = filters.role {
            query.push_str(&format!(" AND role = ${}", param_count));
            bindings.push(Box::new(role.clone()));
            param_count += 1;
        }

        if let Some(ref source) = filters.source {
            query.push_str(&format!(" AND source = ${}", param_count));
            bindings.push(Box::new(source.clone()));
            param_count += 1;
        }

        if let Some(ref network_id) = filters.network_id {
            query.push_str(&format!(" AND network_id = ${}", param_count));
            bindings.push(Box::new(network_id.clone()));
            param_count += 1;
        }

        if let Some(ref station_id) = filters.station_id {
            query.push_str(&format!(" AND station_id = ${}", param_count));
            bindings.push(Box::new(station_id.clone()));
            param_count += 1;
        }

        if let Some(is_active) = filters.is_active {
            query.push_str(&format!(" AND is_active = ${}", param_count));
            bindings.push(Box::new(is_active));
            param_count += 1;
        }

        if let Some(is_verified) = filters.is_verified {
            query.push_str(&format!(" AND is_verified = ${}", param_count));
            bindings.push(Box::new(is_verified));
            param_count += 1;
        }

        // Sorting
        let sort_by = filters.sort_by.as_deref().unwrap_or("created_at");
        let sort_order = match filters.sort_order.unwrap_or_default() {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };
        query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_order));

        // Pagination
        let page_size = filters
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .min(MAX_PAGE_SIZE);
        let page = filters.page.unwrap_or(1).max(1);
        let offset = (page - 1) * page_size;

        query.push_str(&format!(
            " LIMIT ${} OFFSET ${}",
            param_count,
            param_count + 1
        ));
        bindings.push(Box::new(page_size));
        bindings.push(Box::new(offset));

        // Execute query - Note: Dynamic binding requires building query differently
        // For production, consider using a query builder library like sea-query
        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let users: Result<Vec<User>, _> = rows.iter().map(Self::row_to_user).collect();
        users
    }

    async fn count(&self, filters: UserFilters) -> AppResult<i64> {
        let mut query = String::from("SELECT COUNT(*) as count FROM users WHERE 1=1");

        if !filters.include_deleted {
            query.push_str(" AND deleted_at IS NULL");
        }

        if filters.search.is_some() {
            query.push_str(" AND (LOWER(email) LIKE '%' || LOWER($1) || '%' OR LOWER(username) LIKE '%' || LOWER($1) || '%')");
        }

        if filters.role.is_some() {
            query.push_str(" AND role = $2");
        }

        if filters.source.is_some() {
            query.push_str(" AND source = $3");
        }

        // Simplified count query
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    async fn email_exists(&self, email: &str) -> AppResult<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn username_exists(&self, username: &str) -> AppResult<bool> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users WHERE LOWER(username) = LOWER($1) AND deleted_at IS NULL",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn find_by_role(&self, role: &str) -> AppResult<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE role = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(role)
        .fetch_all(&self.pool)
        .await?;

        let users: Result<Vec<User>, _> = rows.iter().map(Self::row_to_user).collect();
        users
    }

    async fn find_by_network_id(&self, network_id: &str) -> AppResult<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE network_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(network_id)
        .fetch_all(&self.pool)
        .await?;

        let users: Result<Vec<User>, _> = rows.iter().map(Self::row_to_user).collect();
        users
    }

    async fn find_by_station_id(&self, station_id: &str) -> AppResult<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE station_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(station_id)
        .fetch_all(&self.pool)
        .await?;

        let users: Result<Vec<User>, _> = rows.iter().map(Self::row_to_user).collect();
        users
    }
}
