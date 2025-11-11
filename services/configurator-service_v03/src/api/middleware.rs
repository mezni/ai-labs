use actix_cors::Cors;
use actix_web::{HttpMessage, dev::ServiceRequest};
use tracing::{error, info};

/// Simple CORS configuration
pub fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}

/// Request logging middleware
pub fn logging_middleware() -> actix_web::middleware::Logger {
    actix_web::middleware::Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
        .log_target("api")
}

/// Error handler middleware
pub fn error_handler_middleware() -> actix_web::middleware::ErrorHandlers<actix_web::body::BoxBody>
{
    actix_web::middleware::ErrorHandlers::new()
        .handler(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            internal_server_error,
        )
        .handler(actix_web::http::StatusCode::NOT_FOUND, not_found_error)
}

async fn internal_server_error(
    res: actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
) -> Result<actix_web::middleware::ErrorHandlerResponse<actix_web::body::BoxBody>, actix_web::Error>
{
    let (req, res) = res.into_parts();

    error!(
        "Internal server error for request: {} {}",
        req.method(),
        req.path()
    );

    let res = res.set_body(actix_web::body::BoxBody::new(
        serde_json::json!({
            "error": "Internal server error",
            "code": 500,
            "request_id": "unknown"
        })
        .to_string(),
    ));

    Ok(actix_web::middleware::ErrorHandlerResponse::Response(
        actix_web::dev::ServiceResponse::new(req, res),
    ))
}

async fn not_found_error(
    res: actix_web::dev::ServiceRequest,
) -> Result<actix_web::middleware::ErrorHandlerResponse<actix_web::body::BoxBody>, actix_web::Error>
{
    let (req, res) = res.into_parts();

    info!("Resource not found: {} {}", req.method(), req.path());

    let res = res.set_body(actix_web::body::BoxBody::new(
        serde_json::json!({
            "error": "Resource not found",
            "code": 404,
            "path": req.path()
        })
        .to_string(),
    ));

    Ok(actix_web::middleware::ErrorHandlerResponse::Response(
        actix_web::dev::ServiceResponse::new(req, res),
    ))
}

/// User information for request context
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
}

/// Simple middleware to add default user info
pub fn default_user_middleware() -> actix_web::middleware::DefaultHeaders {
    actix_web::middleware::DefaultHeaders::new()
        .add(("X-API-Version", "1.0"))
        .add(("X-Content-Type-Options", "nosniff"))
}

/// Simple request validation middleware
pub async fn request_validator(
    req: ServiceRequest,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // Add default user info for now
    // In production, you would validate JWT tokens or session cookies here
    req.extensions_mut().insert(UserInfo {
        user_id: "system".to_string(),
    });

    // Log incoming request
    info!(
        "Incoming request: {} {} from {}",
        req.method(),
        req.path(),
        req.connection_info().peer_addr().unwrap_or("unknown")
    );

    Ok(req)
}

/// Compression middleware configuration
pub fn compression_middleware() -> actix_web::middleware::Compress {
    actix_web::middleware::Compress::default()
}

/// Normalize path middleware (removes trailing slashes)
pub fn normalize_path_middleware() -> actix_web::middleware::NormalizePath {
    actix_web::middleware::NormalizePath::trim()
}
