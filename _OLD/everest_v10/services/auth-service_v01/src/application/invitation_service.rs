use crate::core::errors::{AppError, AppResult};
use crate::domain::entities::{Invitation, User};
use crate::domain::enums::{InvitationStatus, Source, UserRole};
use crate::domain::repositories::{InvitationRepository, UserRepository};
use crate::domain::services::InvitationService as InvitationServiceTrait;
use crate::infrastructure::keycloak_client::KeycloakClient;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

pub struct InvitationService {
    state: Arc<AppState>,
}

impl InvitationService {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl InvitationServiceTrait for InvitationService {
    async fn create_invitation(
        &self,
        email: String,
        role: String,
        invited_by: String,
        expires_in_hours: i64,
        metadata: Option<serde_json::Value>,
    ) -> AppResult<Invitation> {
        // Check if user already exists
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("User already exists".into()));
        }

        // Check for existing pending invitations
        let existing = self.invitation_repo.find_by_email(&email).await?;
        for inv in existing {
            if inv.status == InvitationStatus::Pending && inv.expires_at > Utc::now() {
                return Err(AppError::Conflict("Active invitation already exists".into()));
            }
        }

        let user_role = match role.as_str() {
            "admin" => UserRole::Admin,
            "partner" => UserRole::Partner,
            "operator" => UserRole::Operator,
            _ => UserRole::User,
        };

        let invitation = Invitation {
            invitation_id: nanoid::nanoid!(32),
            code: nanoid::nanoid!(16),
            email,
            role: user_role,
            invited_by,
            status: InvitationStatus::Pending,
            expires_at: Utc::now() + chrono::Duration::hours(expires_in_hours),
            accepted_at: None,
            accepted_by: None,
            created_at: Utc::now(),
            metadata,
        };

        self.invitation_repo.create(&invitation).await
    }

    async fn list_invitations(&self, limit: i64, offset: i64) -> AppResult<Vec<Invitation>> {
        self.invitation_repo.list(limit, offset).await
    }

    async fn get_invitation(&self, code: String) -> AppResult<Invitation> {
        let invitation = self
            .invitation_repo
            .find_by_code(&code)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        // Check if expired
        if invitation.expires_at < Utc::now() {
            return Err(AppError::BadRequest("Invitation expired".into()));
        }

        // Check if already accepted
        if invitation.status == InvitationStatus::Accepted {
            return Err(AppError::BadRequest("Invitation already accepted".into()));
        }

        Ok(invitation)
    }

    async fn accept_invitation(&self, code: String, password: String) -> AppResult<User> {
        let mut invitation = self
            .invitation_repo
            .find_by_code(&code)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        // Validate invitation
        if invitation.status != InvitationStatus::Pending {
            return Err(AppError::BadRequest("Invitation not available".into()));
        }

        if invitation.expires_at < Utc::now() {
            invitation.status = InvitationStatus::Expired;
            self.invitation_repo.update(&invitation).await?;
            return Err(AppError::BadRequest("Invitation expired".into()));
        }

        // Check if user already exists
        if self.user_repo.find_by_email(&invitation.email).await?.is_some() {
            return Err(AppError::Conflict("User already exists".into()));
        }

        // Generate username from email
        let username = invitation.email.split('@').next().unwrap_or("user").to_string();

        // Create user in Keycloak
        let keycloak_id = self
            .keycloak
            .create_user(&invitation.email, &username, &password, None)
            .await
            .map_err(|e| AppError::Keycloak(e.to_string()))?;

        // Assign role
        if invitation.role != UserRole::User {
            self.keycloak
                .assign_role(&keycloak_id, &format!("{:?}", invitation.role).to_lowercase())
                .await
                .map_err(|e| AppError::Keycloak(e.to_string()))?;
        }

        // Create user in database
        let user = User {
            user_id: nanoid::nanoid!(32),
            keycloak_id,
            email: invitation.email.clone(),
            username,
            first_name: None,
            last_name: None,
            phone: None,
            photo: None,
            is_verified: true,
            role: invitation.role.clone(),
            network_id: String::new(),
            station_id: String::new(),
            source: Source::Internal,
            is_active: true,
            deleted_at: None,
            last_login_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: Some(invitation.invited_by.clone()),
            updated_by: None,
        };

        let created_user = self.user_repo.create(&user).await?;

        // Update invitation
        invitation.status = InvitationStatus::Accepted;
        invitation.accepted_at = Some(Utc::now());
        invitation.accepted_by = Some(created_user.user_id.clone());
        self.invitation_repo.update(&invitation).await?;

        Ok(created_user)
    }

    async fn cancel_invitation(&self, code: String) -> AppResult<()> {
        let mut invitation = self
            .invitation_repo
            .find_by_code(&code)
            .await?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        if invitation.status != InvitationStatus::Pending {
            return Err(AppError::BadRequest("Invitation cannot be cancelled".into()));
        }

        invitation.status = InvitationStatus::Cancelled;
        self.invitation_repo.update(&invitation).await?;

        Ok(())
    }
}
