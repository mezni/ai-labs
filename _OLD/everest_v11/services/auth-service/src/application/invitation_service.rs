use std::sync::Arc;

use crate::application::dtos::invitation::{
    AcceptInvitationRequest, CreateInvitationRequest, InvitationDetailResponse, InvitationResponse,
};
use crate::core::{
    constants::*,
    errors::{AppError, AppResult},
    utils::generate_invitation_code,
};
use crate::domain::{entities::Invitation, repositories::{InvitationRepository, UserRepository}};
use crate::infrastructure::keycloak_client::KeycloakClient;

pub struct InvitationService {
    invitation_repo: Arc<dyn InvitationRepository>,
    user_repo: Arc<dyn UserRepository>,
    keycloak_client: Arc<dyn KeycloakClient>,
}

impl InvitationService {
    pub fn new(
        invitation_repo: Arc<dyn InvitationRepository>,
        user_repo: Arc<dyn UserRepository>,
        keycloak_client: Arc<dyn KeycloakClient>,
    ) -> Self {
        Self {
            invitation_repo,
            user_repo,
            keycloak_client,
        }
    }

    pub async fn create_invitation(
        &self,
        req: CreateInvitationRequest,
    ) -> AppResult<InvitationResponse> {
        tracing::info!("Creating invitation for: {}", req.email);

        let code = generate_invitation_code();
        let role = req.role.as_deref().unwrap_or(ROLE_USER);
        let expires_at = chrono::Utc::now() + chrono::Duration::days(INVITATION_EXPIRY_DAYS);

        self.invitation_repo
            .create(&code, &req.email, role, &req.created_by, expires_at)
            .await?;

        tracing::info!("Invitation created with code: {}", code);

        Ok(InvitationResponse {
            code,
            email: req.email,
            role: role.to_string(),
            expires_at,
        })
    }

    pub async fn list_invitations(&self) -> AppResult<Vec<InvitationDetailResponse>> {
        tracing::info!("Listing all invitations");

        let invitations = self.invitation_repo.find_all().await?;

        Ok(invitations.into_iter().map(|i| i.into()).collect())
    }

    pub async fn get_invitation(&self, code: &str) -> AppResult<InvitationDetailResponse> {
        tracing::info!("Getting invitation: {}", code);

        let invitation = self.invitation_repo.find_by_code(code).await?;

        // Check if expired
        if chrono::Utc::now() > invitation.expires_at {
            return Err(AppError::InvitationExpired);
        }

        // Check if already accepted
        if invitation.status == INVITATION_STATUS_ACCEPTED {
            return Err(AppError::InvitationAlreadyAccepted);
        }

        Ok(invitation.into())
    }

    pub async fn accept_invitation(
        &self,
        code: &str,
        req: AcceptInvitationRequest,
    ) -> AppResult<()> {
        tracing::info!("Accepting invitation: {}", code);

        let mut invitation = self.invitation_repo.find_by_code(code).await?;

        // Check if expired
        if chrono::Utc::now() > invitation.expires_at {
            return Err(AppError::InvitationExpired);
        }

        // Check if already accepted
        if invitation.status == INVITATION_STATUS_ACCEPTED {
            return Err(AppError::InvitationAlreadyAccepted);
        }

        // Create user in Keycloak
        let keycloak_id = self
            .keycloak_client
            .create_user(&invitation.email, &req.username, &req.password, None)
            .await
            .map_err(|e| AppError::KeycloakError(e.to_string()))?;

        // Assign role
        if invitation.role == ROLE_ADMIN {
            self.keycloak_client
                .assign_role(&keycloak_id, ROLE_ADMIN)
                .await
                .map_err(|e| AppError::KeycloakError(e.to_string()))?;
        }

        // Create user in database
        let user = self
            .user_repo
            .create(&keycloak_id, &invitation.email, &req.username, &invitation.role)
            .await?;

        // Update invitation status
        invitation.status = INVITATION_STATUS_ACCEPTED.to_string();
        invitation.accepted_by = Some(user.id);
        invitation.updated_at = chrono::Utc::now();

        self.invitation_repo.update(&invitation).await?;

        tracing::info!("Invitation accepted successfully: {}", code);

        Ok(())
    }

    pub async fn cancel_invitation(&self, code: &str) -> AppResult<()> {
        tracing::info!("Cancelling invitation: {}", code);

        let mut invitation = self.invitation_repo.find_by_code(code).await?;

        invitation.status = INVITATION_STATUS_CANCELLED.to_string();
        invitation.updated_at = chrono::Utc::now();

        self.invitation_repo.update(&invitation).await?;

        tracing::info!("Invitation cancelled: {}", code);

        Ok(())
    }
}

impl From<Invitation> for InvitationDetailResponse {
    fn from(inv: Invitation) -> Self {
        Self {
            id: inv.id,
            code: inv.code,
            email: inv.email,
            role: inv.role,
            status: inv.status,
            created_by: inv.created_by,
            accepted_by: inv.accepted_by,
            expires_at: inv.expires_at,
            created_at: inv.created_at,
        }
    }
}