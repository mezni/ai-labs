// src/jobs/keycloak_sync.rs
use crate::core::{AppError, constants::*, errors::AppResult};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::KeycloakClient;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub struct KeycloakSyncJob {
    pool: PgPool,
    user_repo: Arc<dyn UserRepository>,
    keycloak_client: Arc<KeycloakClient>,
    sync_interval: Duration,
}

#[derive(Debug)]
pub struct SyncResult {
    pub synced: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl KeycloakSyncJob {
    pub fn new(
        pool: PgPool,
        user_repo: Arc<dyn UserRepository>,
        keycloak_client: Arc<KeycloakClient>,
        sync_interval_secs: u64,
    ) -> Self {
        Self {
            pool,
            user_repo,
            keycloak_client,
            sync_interval: Duration::from_secs(sync_interval_secs),
        }
    }

    /// Start the sync job
    pub async fn start(self: Arc<Self>) {
        let mut interval = interval(self.sync_interval);

        tracing::info!(
            "Keycloak sync job started with interval: {:?}",
            self.sync_interval
        );

        loop {
            interval.tick().await;

            match self.run_sync().await {
                Ok(result) => {
                    tracing::info!(
                        "Keycloak sync completed: {} synced, {} failed, {} skipped",
                        result.synced,
                        result.failed,
                        result.skipped
                    );
                }
                Err(e) => {
                    tracing::error!("Keycloak sync job failed: {}", e);
                }
            }
        }
    }

    /// Run a single sync cycle
    async fn run_sync(&self) -> AppResult<SyncResult> {
        let mut result = SyncResult {
            synced: 0,
            failed: 0,
            skipped: 0,
        };

        // Get all active users from database
        let users = sqlx::query!(
            r#"
            SELECT user_id, keycloak_id, email, username, role, 
                   is_active as "is_active!"
            FROM users
            WHERE deleted_at IS NULL
            ORDER BY updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        tracing::debug!("Found {} users to sync", users.len());

        for user in users {
            match self
                .sync_user(&user.user_id, &user.keycloak_id, &user.role, user.is_active)
                .await
            {
                Ok(true) => result.synced += 1,
                Ok(false) => result.skipped += 1,
                Err(e) => {
                    tracing::warn!(
                        "Failed to sync user {} ({}): {}",
                        user.user_id,
                        user.keycloak_id,
                        e
                    );
                    result.failed += 1;

                    // Log sync failure
                    let _ = self
                        .log_sync_failure(
                            &user.user_id,
                            &user.keycloak_id,
                            SYNC_ACTION_UPDATE,
                            &e.to_string(),
                        )
                        .await;
                }
            }
        }

        Ok(result)
    }

    /// Sync a single user
    async fn sync_user(
        &self,
        user_id: &str,
        keycloak_id: &str,
        db_role: &str,
        db_is_active: bool,
    ) -> AppResult<bool> {
        // Get user from Keycloak
        let keycloak_user = match self.keycloak_client.get_user(keycloak_id).await {
            Ok(user) => user,
            Err(AppError::NotFound(_)) => {
                // User doesn't exist in Keycloak anymore
                tracing::warn!(
                    "User {} not found in Keycloak, marking as inactive",
                    keycloak_id
                );

                // Mark user as inactive in database
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET is_active = false, updated_at = NOW()
                    WHERE user_id = $1
                    "#,
                    user_id
                )
                .execute(&self.pool)
                .await?;

                self.log_sync_success(
                    user_id,
                    keycloak_id,
                    SYNC_ACTION_STATUS_UPDATE,
                    "User not found in Keycloak, marked as inactive",
                )
                .await?;

                return Ok(true);
            }
            Err(e) => return Err(e),
        };

        let mut changes = Vec::new();
        let mut needs_update = false;

        // Check if status changed
        if keycloak_user.enabled != db_is_active {
            needs_update = true;
            changes.push(format!(
                "enabled: {} -> {}",
                db_is_active, keycloak_user.enabled
            ));

            // Update database
            sqlx::query!(
                r#"
                UPDATE users
                SET is_active = $2, updated_at = NOW()
                WHERE user_id = $1
                "#,
                user_id,
                keycloak_user.enabled
            )
            .execute(&self.pool)
            .await?;
        }

        // Check if email verification changed
        if keycloak_user.email_verified {
            let verified_in_db: bool = sqlx::query_scalar!(
                r#"SELECT is_verified as "is_verified!" FROM users WHERE user_id = $1"#,
                user_id
            )
            .fetch_one(&self.pool)
            .await?;

            if !verified_in_db {
                needs_update = true;
                changes.push("email_verified: false -> true".to_string());

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
            }
        }

        if needs_update {
            self.log_sync_success(
                user_id,
                keycloak_id,
                SYNC_ACTION_UPDATE,
                &format!("Synced changes: {}", changes.join(", ")),
            )
            .await?;

            Ok(true)
        } else {
            Ok(false) // No changes needed
        }
    }

    /// Log successful sync
    async fn log_sync_success(
        &self,
        user_id: &str,
        keycloak_id: &str,
        action: &str,
        details: &str,
    ) -> AppResult<()> {
        let now = Utc::now().naive_utc();

        sqlx::query!(
            r#"
            INSERT INTO keycloak_sync_log (
                user_id, keycloak_id, action, status, details, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_id,
            keycloak_id,
            action,
            SYNC_STATUS_SUCCESS,
            details,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Log failed sync
    async fn log_sync_failure(
        &self,
        user_id: &str,
        keycloak_id: &str,
        action: &str,
        error_message: &str,
    ) -> AppResult<()> {
        let now = Utc::now().naive_utc();

        sqlx::query!(
            r#"
            INSERT INTO keycloak_sync_log (
                user_id, keycloak_id, action, status, error_message, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_id,
            keycloak_id,
            action,
            SYNC_STATUS_FAILED,
            error_message,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Manually trigger a sync (useful for testing or admin endpoint)
    pub async fn trigger_sync(&self) -> AppResult<SyncResult> {
        tracing::info!("Manual Keycloak sync triggered");
        self.run_sync().await
    }

    /// Sync a specific user by ID
    pub async fn sync_user_by_id(&self, user_id: &str) -> AppResult<()> {
        let user = sqlx::query!(
            r#"
            SELECT keycloak_id, role, is_active as "is_active!"
            FROM users
            WHERE user_id = $1 AND deleted_at IS NULL
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        self.sync_user(user_id, &user.keycloak_id, &user.role, user.is_active)
            .await?;

        Ok(())
    }

    /// Get sync statistics
    pub async fn get_sync_stats(&self) -> AppResult<SyncStats> {
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as "total!",
                COUNT(*) FILTER (WHERE status = 'success') as "success!",
                COUNT(*) FILTER (WHERE status = 'failed') as "failed!",
                COUNT(*) FILTER (WHERE status = 'skipped') as "skipped!",
                MAX(created_at) as last_sync_at
            FROM keycloak_sync_log
            WHERE created_at > NOW() - INTERVAL '24 hours'
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SyncStats {
            total: row.total,
            success: row.success,
            failed: row.failed,
            skipped: row.skipped,
            last_sync_at: row
                .last_sync_at
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
        })
    }

    /// Clean up old sync logs (keep last 30 days)
    pub async fn cleanup_old_logs(&self) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM keycloak_sync_log
            WHERE created_at < NOW() - INTERVAL '30 days'
            "#
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("Cleaned up {} old sync logs", result.rows_affected());

        Ok(result.rows_affected())
    }
}

#[derive(Debug)]
pub struct SyncStats {
    pub total: i64,
    pub success: i64,
    pub failed: i64,
    pub skipped: i64,
    pub last_sync_at: Option<chrono::DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_result_creation() {
        let result = SyncResult {
            synced: 10,
            failed: 2,
            skipped: 5,
        };

        assert_eq!(result.synced, 10);
        assert_eq!(result.failed, 2);
        assert_eq!(result.skipped, 5);
    }
}
