use crate::{
    application::{dtos::*, services::AuthService},
    core::{config::Config, errors::AppError, jwt},
    infrastructure::{cache::JwtKeyCache, keycloak::KeycloakClient, persistence::PostgresRepository},
};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use jsonwebtoken::DecodingKey;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use tracing::info;
use validator::Validate;

fn get_ip(req: &HttpRequest) -> Option<String> {
    req.connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
}

async fn get_jwks_key(
    config: &Config,
    cache: &JwtKeyCache,
    kid: &str,
) -> Result<DecodingKey, AppError> {
    if let Some(key) = cache.get(kid) {
        return Ok(key);
    }

    let jwks_url = config.keycloak_jwks_url();
    let response = reqwest::get(&jwks_url)
        .await
        .map_err(|e| AppError::Internal(format!("JWKS fetch failed: {}", e)))?;

    let jwks: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("JWKS parse failed: {}", e)))?;

    let keys = jwks["keys"]
        .as_array()
        .ok_or_else(|| AppError::Internal("Invalid JWKS format".to_string()))?;

    for key in keys {
        if key["kid"].as_str() == Some(kid) {
            if let (Some(n), Some(e)) = (key["n"].as_str(), key["e"].as_str()) {
                let decoding_key = DecodingKey::from_rsa_components(n, e)
                    .map_err(|e| AppError::Internal(format!("Key decode failed: {}", e)))?;

                cache.set(kid.to_string(), decoding_key.clone(), Duration::from_secs(3600));
                return Ok(decoding_key);
            }
        }
    }

    Err(AppError::Unauthorized("Key not found in JWKS".to_string()))
}

async fn extract_claims(
    req: &HttpRequest,
    config: &Config,
    cache: &JwtKeyCache,
) -> Result<jwt::Claims, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = jwt::extract_bearer_token(auth_header)?;

    let header = jsonwebtoken::decode_header(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    let kid = header
        .kid
        .ok_or_else(|| AppError::Unauthorized("Missing kid in token".to_string()))?;

    let key = get_jwks_key(config, cache, &kid).await?;
    let claims = jwt::validate_token(token, &key, &config.keycloak_issuer())?;

    Ok(claims)
}

fn require_admin(claims: &jwt::Claims) -> Result<(), AppError> {
    if !claims.is_admin() {
        return Err(AppError::Forbidden("Admin role required".to_string()));
    }
    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Email already exists")
    ),
    tag = "auth"
)]
pub async fn register(
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());
    let ip = get_ip(&req);

    let response = service.register_external_user(body.into_inner(), ip).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "User profile", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "users"
)]
pub async fn get_me(
    req: HttpRequest,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &config, &cache).await?;
    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());

    let user = service.get_user(&claims.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/me",
    request_body = UpdateMeRequest,
    responses(
        (status = 200, description = "Profile updated", body = UserResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "users"
)]
pub async fn update_me(
    req: HttpRequest,
    body: web::Json<UpdateMeRequest>,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let claims = extract_claims(&req, &config, &cache).await?;
    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());

    let user = service.update_me(&claims.user_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    request_body = CreateInternalUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "admin"
)]
pub async fn create_internal_user(
    req: HttpRequest,
    body: web::Json<CreateInternalUserRequest>,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let claims = extract_claims(&req, &config, &cache).await?;
    require_admin(&claims)?;

    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());
    let ip = get_ip(&req);

    let user = service
        .create_internal_user(body.into_inner(), &claims.user_id, ip)
        .await?;

    Ok(HttpResponse::Created().json(user))
}

#[derive(Deserialize)]
pub struct ListUsersQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    search: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    params(
        ("limit" = Option<i64>, Query, description = "Number of users to return"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination"),
        ("search" = Option<String>, Query, description = "Search term for email")
    ),
    responses(
        (status = 200, description = "List of users", body = ListUsersResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "admin"
)]
pub async fn list_users(
    req: HttpRequest,
    query: web::Query<ListUsersQuery>,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &config, &cache).await?;
    require_admin(&claims)?;

    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let response = service
        .list_users(limit, offset, query.search.clone())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    put,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "admin"
)]
pub async fn update_user(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let claims = extract_claims(&req, &config, &cache).await?;
    require_admin(&claims)?;

    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());
    let ip = get_ip(&req);

    let user = service
        .update_user(&path.into_inner(), body.into_inner(), &claims.user_id, ip)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer" = [])
    ),
    tag = "admin"
)]
pub async fn delete_user(
    req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<Config>,
    cache: web::Data<Arc<JwtKeyCache>>,
    repo: web::Data<Arc<PostgresRepository>>,
    keycloak: web::Data<Arc<KeycloakClient>>,
) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req, &config, &cache).await?;
    require_admin(&claims)?;

    let service = AuthService::new(repo.get_ref().clone(), repo.get_ref().clone(), keycloak.get_ref().clone());
    let ip = get_ip(&req);

    service
        .delete_user(&path.into_inner(), &claims.user_id, ip)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}