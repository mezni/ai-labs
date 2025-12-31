use crate::domain::{
    entities::{AuditLog, Role, User, UserSource},
    repositories::{AuditRepository, UserRepository},
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row_to_user(row: &sqlx::postgres::PgRow) -> Result<User, sqlx::Error> {
        use sqlx::Row;

        let source_str: String = row.try_get("source")?;
        let roles_str: Vec<String> = row.try_get("roles")?;

        Ok(User {
            id: row.try_get("id")?,
            keycloak_id: row.try_get("keycloak_id")?,
            email: row.try_get("email")?,
            source: match source_str.as_str() {
                "web" => UserSource::Web,
                "internal" => UserSource::Internal,
                _ => UserSource::Web,
            },
            roles: roles_str
                .iter()
                .filter_map(|r| Role::from_str(r))
                .collect(),
            network_id: row.try_get("network_id")?,
            station_id: row.try_get("station_id")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let source_str = match user.source {
            UserSource::Web => "web",
            UserSource::Internal => "internal",
        };

        let roles_str: Vec<String> = user.roles.iter().map(|r| r.as_str().to_string()).collect();

        let row = sqlx::query(
            r#"
            INSERT INTO users (id, keycloak_id, email, source, roles, network_id, station_id, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(&user.id)
        .bind(&user.keycloak_id)
        .bind(&user.email)
        .bind(source_str)
        .bind(&roles_str)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Self::map_row_to_user(&row)
    }

    async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        row.as_ref().map(Self::map_row_to_user).transpose()
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        row.as_ref().map(Self::map_row_to_user).transpose()
    }

    async fn find_user_by_keycloak_id(
        &self,
        keycloak_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM users WHERE keycloak_id = $1")
            .bind(keycloak_id)
            .fetch_optional(&self.pool)
            .await?;

        row.as_ref().map(Self::map_row_to_user).transpose()
    }

    async fn list_users(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<User>, sqlx::Error> {
        let rows = if let Some(search_term) = search {
            sqlx::query(
                "SELECT * FROM users WHERE deleted_at IS NULL AND email ILIKE $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(format!("%{}%", search_term))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                "SELECT * FROM users WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        rows.iter().map(Self::map_row_to_user).collect()
    }

    async fn update_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let source_str = match user.source {
            UserSource::Web => "web",
            UserSource::Internal => "internal",
        };

        let roles_str: Vec<String> = user.roles.iter().map(|r| r.as_str().to_string()).collect();

        let row = sqlx::query(
            r#"
            UPDATE users 
            SET email = $1, source = $2, roles = $3, network_id = $4, station_id = $5, 
                is_active = $6, updated_at = $7
            WHERE id = $8
            RETURNING *
            "#,
        )
        .bind(&user.email)
        .bind(source_str)
        .bind(&roles_str)
        .bind(&user.network_id)
        .bind(&user.station_id)
        .bind(user.is_active)
        .bind(Utc::now())
        .bind(&user.id)
        .fetch_one(&self.pool)
        .await?;

        Self::map_row_to_user(&row)
    }

    async fn soft_delete_user(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET deleted_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl AuditRepository for PostgresRepository {
    async fn create_audit_log(&self, log: &AuditLog) -> Result<AuditLog, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, details, ip_address, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&log.id)
        .bind(&log.user_id)
        .bind(&log.action)
        .bind(&log.resource_type)
        .bind(&log.resource_id)
        .bind(&log.details)
        .bind(&log.ip_address)
        .bind(log.created_at)
        .execute(&self.pool)
        .await?;

        Ok(log.clone())
    }
}