use crate::application::dtos::invitation::*;
use crate::core::errors::AppError;
use crate::domain::services::InvitationService;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, ResponseError};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/v1/invitations",
    request_body = CreateInvitationRequest,
    responses(
        (status = 201, description = "Invitation created", body = InvitationResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists or active invitation exists")
    ),
    tag = "Invitations",
    security(("bearer_auth" = []))
)]
#[post("/invitations")]
pub async fn create_invitation(
    service: web::Data<Arc<dyn InvitationService>>,
    req: HttpRequest,
    body: web::Json<CreateInvitationRequest>,
) -> HttpResponse {
    if let Err(e) = body.0.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": e.to_string()
        }));
    }

    // Extract user_id from token (simplified - in real app, extract from JWT)
    let invited_by = "admin".to_string(); // TODO: Extract from JWT token

    match service
        .create_invitation(
            body.email.clone(),
            body.role.clone(),
            invited_by,
            body.expires_in_hours,
            body.metadata.clone(),
        )
        .await
    {
        Ok(invitation) => HttpResponse::Created().json(InvitationResponse {
            invitation_id: invitation.invitation_id,
            code: invitation.code,
            email: invitation.email,
            role: format!("{:?}", invitation.role),
            status: format!("{:?}", invitation.status),
            expires_at: invitation.expires_at.to_rfc3339(),
            created_at: invitation.created_at.to_rfc3339(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/invitations",
    params(
        ("limit" = Option<i64>, Query, description = "Number of invitations to return"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination")
    ),
    responses(
        (status = 200, description = "Invitations retrieved", body = InvitationListResponse)
    ),
    tag = "Invitations",
    security(("bearer_auth" = []))
)]
#[get("/invitations")]
pub async fn list_invitations(
    service: web::Data<Arc<dyn InvitationService>>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    match service.list_invitations(limit, offset).await {
        Ok(invitations) => {
            let response = InvitationListResponse {
                total: invitations.len(),
                invitations: invitations
                    .into_iter()
                    .map(|inv| InvitationResponse {
                        invitation_id: inv.invitation_id,
                        code: inv.code,
                        email: inv.email,
                        role: format!("{:?}", inv.role),
                        status: format!("{:?}", inv.status),
                        expires_at: inv.expires_at.to_rfc3339(),
                        created_at: inv.created_at.to_rfc3339(),
                    })
                    .collect(),
                limit,
                offset,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => e.error_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/v1/invitations/{code}",
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    responses(
        (status = 200, description = "Invitation found", body = InvitationResponse),
        (status = 404, description = "Invitation not found"),
        (status = 400, description = "Invitation expired or already used")
    ),
    tag = "Invitations"
)]
#[get("/invitations/{code}")]
pub async fn get_invitation(
    service: web::Data<Arc<dyn InvitationService>>,
    path: web::Path<String>,
) -> HttpResponse {
    match service.get_invitation(path.into_inner()).await {
        Ok(invitation) => HttpResponse::Ok().json(InvitationResponse {
            invitation_id: invitation.invitation_id,
            code: invitation.code,
            email: invitation.email,
            role: format!("{:?}", invitation.role),
            status: format!("{:?}", invitation.status),
            expires_at: invitation.expires_at.to_rfc3339(),
            created_at: invitation.created_at.to_rfc3339(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/invitations/{code}/accept",
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    request_body = AcceptInvitationRequest,
    responses(
        (status = 200, description = "Invitation accepted", body = AcceptInvitationResponse),
        (status = 400, description = "Invalid invitation or password"),
        (status = 404, description = "Invitation not found"),
        (status = 409, description = "User already exists")
    ),
    tag = "Invitations"
)]
#[post("/invitations/{code}/accept")]
pub async fn accept_invitation(
    service: web::Data<Arc<dyn InvitationService>>,
    path: web::Path<String>,
    body: web::Json<AcceptInvitationRequest>,
) -> HttpResponse {
    if let Err(e) = body.0.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": e.to_string()
        }));
    }

    match service
        .accept_invitation(path.into_inner(), body.password.clone())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(AcceptInvitationResponse {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            message: "Invitation accepted successfully. You can now login.".to_string(),
        }),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/invitations/{code}",
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    responses(
        (status = 200, description = "Invitation cancelled"),
        (status = 404, description = "Invitation not found"),
        (status = 400, description = "Invitation cannot be cancelled")
    ),
    tag = "Invitations",
    security(("bearer_auth" = []))
)]
#[delete("/invitations/{code}")]
pub async fn cancel_invitation(
    service: web::Data<Arc<dyn InvitationService>>,
    path: web::Path<String>,
) -> HttpResponse {
    match service.cancel_invitation(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Invitation cancelled successfully"
        })),
        Err(e) => e.error_response(),
    }
}

