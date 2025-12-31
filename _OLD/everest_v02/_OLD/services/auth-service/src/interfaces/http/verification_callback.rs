use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::application::verification_callback_service::VerificationCallbackService;
use crate::core::{errors::AppError, AppState};
use crate::infrastructure::repositories_pg::{PgRegistrationRepository, PgUserRepository};

#[derive(Debug, Deserialize, ToSchema)]
pub struct KeycloakVerificationCallback {
    #[schema(example = "9b7e4f9c-1c9d-4b1f-9c44-abc123")]
    pub keycloak_id: String,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = true)]
    pub verified: bool,
    pub verified_at: Option<String>,
}

/// Keycloak email verification callback
///
/// Callback endpoint invoked by Keycloak after a user verifies their email.
/// This endpoint finalizes user creation.
#[utoipa::path(
    post,
    path = "/api/v1/verify/callback",
    request_body = KeycloakVerificationCallback,
    responses(
        (status = 204, description = "Verification processed successfully"),
        (status = 400, description = "Invalid callback payload"),
        (status = 404, description = "Registration not found"),
        (status = 409, description = "User already verified"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Verification",
    security(
        ("keycloakWebhook" = [])
    )
)]
pub async fn verify_callback(
    state: web::Data<AppState>,
    req: web::Json<KeycloakVerificationCallback>,
) -> Result<HttpResponse, AppError> {
    // Validate callback data
    if !req.verified {
        return Err(AppError::ValidationError("User not verified".to_string()));
    }

    // Create service
    let registration_repo = PgRegistrationRepository::new(state.db_pool.clone());
    let user_repo = PgUserRepository::new(state.db_pool.clone());
    let service = VerificationCallbackService::new(
        registration_repo,
        user_repo,
        state.keycloak_client.clone(),
        state.db_pool.clone(),
    );

    // Handle verification
    service
        .handle_verification(req.keycloak_id.clone(), req.email.clone())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
