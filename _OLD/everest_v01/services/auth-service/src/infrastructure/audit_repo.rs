// src/infrastructure/audit_repo.rs
use crate::core::{constants::*, errors::AppResult};
use crate::domain::repositories::{AuditLogFilters, AuditLogRepository, LoginAuditLog};
use async_trait::async_trait;
use sqlx::{PgPool, Row};

pub struct PostgresAuditLogRepository {
    pool: PgPool,
}

impl PostgresAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for PostgresAuditLogRepository {
    async fn log_login(
        &self,
        user_id: &str,
        keycloak_id: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
        success: bool,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, keycloak_id, action, ip_address, user_agent, success, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            "#,
        )
        .bind(user_id)
        .bind(keycloak_id)
        .bind(ACTION_LOGIN)
        .bind(ip_address)
        .bind(user_agent)
        .bind(success)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_logout(&self, user_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, success, created_at
            )
            VALUES ($1, $2, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_LOGOUT)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_password_change(&self, user_id: &str, changed_by: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_PASSWORD_CHANGE)
        .bind(format!("Changed by: {}", changed_by))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_password_reset_request(
        &self,
        user_id: &str,
        email: &str,
        ip_address: Option<String>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, ip_address, success, created_at
            )
            VALUES ($1, $2, $3, $4, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_PASSWORD_RESET)
        .bind(format!("Reset requested for: {}", email))
        .bind(ip_address)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_password_reset(&self, user_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_PASSWORD_RESET)
        .bind("Password reset completed")
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_email_verification(&self, user_id: &str, email: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_EMAIL_VERIFY)
        .bind(format!("Email verified: {}", email))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_user_creation(
        &self,
        user_id: &str,
        created_by: Option<&str>,
        action_details: &str,
    ) -> AppResult<()> {
        let details = if let Some(creator) = created_by {
            format!("{} | Created by: {}", action_details, creator)
        } else {
            action_details.to_string()
        };

        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_USER_CREATE)
        .bind(details)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_user_update(
        &self,
        user_id: &str,
        updated_by: &str,
        action_details: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_USER_UPDATE)
        .bind(format!("{} | Updated by: {}", action_details, updated_by))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_user_deletion(&self, user_id: &str, deleted_by: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_USER_DELETE)
        .bind(format!("Deleted by: {}", deleted_by))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn log_role_change(
        &self,
        user_id: &str,
        old_role: &str,
        new_role: &str,
        changed_by: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO login_audit_log (
                user_id, action, action_details, success, created_at
            )
            VALUES ($1, $2, $3, true, NOW())
            "#,
        )
        .bind(user_id)
        .bind(ACTION_ROLE_CHANGE)
        .bind(format!(
            "Role changed from '{}' to '{}' by {}",
            old_role, new_role, changed_by
        ))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_login_logs(&self, filters: AuditLogFilters) -> AppResult<Vec<LoginAuditLog>> {
        let mut query = String::from("SELECT * FROM login_audit_log WHERE 1=1");

        if let Some(_) = filters.user_id {
            query.push_str(" AND user_id = $1");
        }

        if let Some(_) = filters.action {
            query.push_str(" AND action = $2");
        }

        if let Some(_) = filters.from_date {
            query.push_str(" AND created_at >= $3");
        }

        if let Some(_) = filters.to_date {
            query.push_str(" AND created_at <= $4");
        }

        if let Some(_) = filters.success {
            query.push_str(" AND success = $5");
        }

        query.push_str(" ORDER BY created_at DESC");

        let page_size = filters
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .min(MAX_PAGE_SIZE);
        let page = filters.page.unwrap_or(1).max(1);
        let offset = (page - 1) * page_size;

        query.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));

        // Simplified query execution
        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let logs = rows
            .iter()
            .map(|row| LoginAuditLog {
                log_id: row.get("log_id"),
                user_id: row.get("user_id"),
                keycloak_id: row
                    .get::<Option<String>, _>("keycloak_id")
                    .unwrap_or_default(),
                action: row.get("action"),
                ip_address: row.get("ip_address"),
                user_agent: row.get("user_agent"),
                success: row.get("success"),
                error_message: row.get("error_message"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(logs)
    }

    async fn count_login_logs(&self, filters: AuditLogFilters) -> AppResult<i64> {
        let mut query = String::from("SELECT COUNT(*) as count FROM login_audit_log WHERE 1=1");

        if filters.user_id.is_some() {
            query.push_str(" AND user_id = $1");
        }

        if filters.action.is_some() {
            query.push_str(" AND action = $2");
        }

        let count: i64 = sqlx::query_scalar(&query).fetch_one(&self.pool).await?;

        Ok(count)
    }
}
