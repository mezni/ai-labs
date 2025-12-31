use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::user_registration_service::UserRegistrationService;
use crate::core::{errors::AppError, AppState};
use crate::infrastructure::repositories_pg::{PgRegistrationRepository, PgUserRepository};

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "johndoe", min_length = 3, max_length = 100)]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
    #[schema(example = "+15551234567")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub registration_id: String,
    pub status: String,
    pub expires_at: String,
}

/// Register a new user (self-registration)
///
/// Creates a pending user registration and triggers Keycloak to send a verification email
#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegisterUserResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Email or username already registered"),
        (status = 429, description = "Too many registration attempts"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Registration"
)]
pub async fn register_user(
    state: web::Data<AppState>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if req.email.is_empty() || req.username.is_empty() {
        return Err(AppError::ValidationError(
            "Email and username are required".to_string(),
        ));
    }

    if req.username.len() < 3 {
        return Err(AppError::ValidationError(
            "Username must be at least 3 characters".to_string(),
        ));
    }

    // Create service
    let registration_repo = PgRegistrationRepository::new(state.db_pool.clone());
    let user_repo = PgUserRepository::new(state.db_pool.clone());
    let service = UserRegistrationService::new(
        registration_repo,
        user_repo,
        state.keycloak_client.clone(),
        state.db_pool.clone(),
    );

    // Register user
    let registration = service
        .register_user(
            req.email.clone(),
            req.username.clone(),
            req.first_name.clone(),
            req.last_name.clone(),
            req.phone.clone(),
        )
        .await?;

    let response = RegisterUserResponse {
        registration_id: registration.registration_id,
        status: registration.status.to_string(),
        expires_at: registration.expires_at.to_rfc3339(),
    };

    Ok(HttpResponse::Created().json(response))
}
