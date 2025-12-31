use actix_web::{delete, get, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    application::dtos::invitation::{AcceptInvitationRequest, CreateInvitationRequest},
    AppState,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_invitation)
        .service(list_invitations)
        .service(get_invitation)
        .service(accept_invitation)
        .service(cancel_invitation);
}

#[utoipa::path(
    post,
    path = "/invitations",
    tag = "Invitations",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateInvitationRequest,
    responses(
        (status = 201, description = "Invitation created", body = InvitationResponse),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required")
    )
)]
#[post("/invitations")]
async fn create_invitation(
    state: web::Data<AppState>,
    req: web::Json<CreateInvitationRequest>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.invitation_service.create_invitation(req.into_inner()).await {
        Ok(invitation) => HttpResponse::Created().json(invitation),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/invitations",
    tag = "Invitations",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of invitations", body = Vec<InvitationDetailResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
#[get("/invitations")]
async fn list_invitations(
    state: web::Data<AppState>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.invitation_service.list_invitations().await {
        Ok(invitations) => HttpResponse::Ok().json(invitations),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    get,
    path = "/invitations/{code}",
    tag = "Invitations",
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    responses(
        (status = 200, description = "Invitation details", body = InvitationDetailResponse),
        (status = 404, description = "Invitation not found"),
        (status = 410, description = "Invitation expired")
    )
)]
#[get("/invitations/{code}")]
async fn get_invitation(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    match state.invitation_service.get_invitation(&path.into_inner()).await {
        Ok(invitation) => HttpResponse::Ok().json(invitation),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/invitations/{code}/accept",
    tag = "Invitations",
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    request_body = AcceptInvitationRequest,
    responses(
        (status = 200, description = "Invitation accepted successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Invitation not found"),
        (status = 409, description = "Invitation already accepted"),
        (status = 410, description = "Invitation expired")
    )
)]
#[post("/invitations/{code}/accept")]
async fn accept_invitation(
    state: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<AcceptInvitationRequest>,
) -> HttpResponse {
    match state.invitation_service
        .accept_invitation(&path.into_inner(), req.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Invitation accepted successfully. You can now log in."
        })),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/invitations/{code}",
    tag = "Invitations",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("code" = String, Path, description = "Invitation code")
    ),
    responses(
        (status = 200, description = "Invitation cancelled"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Invitation not found")
    )
)]
#[delete("/invitations/{code}")]
async fn cancel_invitation(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _auth: BearerAuth,
) -> HttpResponse {
    match state.invitation_service.cancel_invitation(&path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Invitation cancelled successfully"
        })),
        Err(e) => e.error_response(),
    }
}