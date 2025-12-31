use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::{constants::INVITATION_STATUS_PENDING, errors::AppResult};
use crate::domain::{entities::Invitation, repositories::InvitationRepository};

pub struct PostgresInvitationRepository {
    pool: PgPool,
}

impl PostgresInvitationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvitationRepository for PostgresInvitationRepository {
    async fn create(
        &self,
        code: &str,
        email: &str,
        role: &str,
        created_by: &Uuid,
        expires_at: DateTime<Utc>,
    ) -> AppResult<Invitation> {
        let invitation = sqlx::query_as!(
            Invitation,
            r#"
            INSERT INTO invitations (code, email, role, status, created_by, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, code, email, role, status, created_by, accepted_by, expires_at, created_at, updated_at
            "#,
            code,
            email,
            role,
            INVITATION_STATUS_PENDING,
            created_by,
            expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(invitation)
    }

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Invitation> {
        let invitation = sqlx::query_as!(
            Invitation,
            r#"
            SELECT id, code, email, role, status, created_by, accepted_by, expires_at, created_at, updated_at
            FROM invitations
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(invitation)
    }

    async fn find_by_code(&self, code: &str) -> AppResult<Invitation> {
        let invitation = sqlx::query_as!(
            Invitation,
            r#"
            SELECT id, code, email, role, status, created_by, accepted_by, expires_at, created_at, updated_at
            FROM invitations
            WHERE code = $1
            "#,
            code
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(invitation)
    }

    async fn find_all(&self) -> AppResult<Vec<Invitation>> {
        let invitations = sqlx::query_as!(
            Invitation,
            r#"
            SELECT id, code, email, role, status, created_by, accepted_by, expires_at, created_at, updated_at
            FROM invitations
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invitations)
    }

    async fn update(&self, invitation: &Invitation) -> AppResult<Invitation> {
        let updated_invitation = sqlx::query_as!(
            Invitation,
            r#"
            UPDATE invitations
            SET status = $1, accepted_by = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, code, email, role, status, created_by, accepted_by, expires_at, created_at, updated_at
            "#,
            invitation.status,
            invitation.accepted_by,
            invitation.updated_at,
            invitation.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_invitation)
    }
}