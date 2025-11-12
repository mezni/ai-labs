use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

// === CONSTANTS ===
const API_PREFIX: &str = "/api/v1";
const JWT_SECRET: &str = "secret123";
const JWT_EXPIRY_HOURS: u64 = 24;

// === ERROR TYPES ===
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("User already exists")]
    UserExists,

    #[error("User not found")]
    UserNotFound,
}

impl actix_web::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials => {
                HttpResponse::Unauthorized().json("Invalid credentials")
            }
            AuthError::InvalidToken => HttpResponse::Unauthorized().json("Invalid token"),
            AuthError::TokenExpired => HttpResponse::Unauthorized().json("Token expired"),
            AuthError::InsufficientPermissions => {
                HttpResponse::Forbidden().json("Insufficient permissions")
            }
            AuthError::UserExists => HttpResponse::Conflict().json("User already exists"),
            AuthError::UserNotFound => HttpResponse::NotFound().json("User not found"),
        }
    }
}

// === AUTH DATA MODELS ===
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub username: String,
    pub password: String, // In production, store hashed passwords
    pub role: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub role: String,
    pub exp: u64, // expiry timestamp
}

// Response models for API
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub username: String,
    pub role: String,
    pub is_active: bool,
}

// === IN-MEMORY USER REPOSITORY ===
#[derive(Debug, Clone)]
pub struct UserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl UserRepository {
    pub fn new() -> Self {
        let users = Arc::new(RwLock::new(HashMap::new()));

        let repo = Self { users };

        // Add some default users
        repo.add_user(User {
            username: "admin".to_string(),
            password: "admin123".to_string(),
            role: "admin".to_string(),
            is_active: true,
        })
        .unwrap();

        repo.add_user(User {
            username: "user".to_string(),
            password: "user123".to_string(),
            role: "user".to_string(),
            is_active: true,
        })
        .unwrap();

        repo
    }

    pub fn add_user(&self, user: User) -> Result<(), AuthError> {
        let mut users = self.users.write().unwrap();
        if users.contains_key(&user.username) {
            return Err(AuthError::UserExists);
        }
        users.insert(user.username.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, username: &str) -> Option<User> {
        let users = self.users.read().unwrap();
        users.get(username).cloned()
    }

    pub fn validate_credentials(&self, username: &str, password: &str) -> Result<User, AuthError> {
        let users = self.users.read().unwrap();
        if let Some(user) = users.get(username) {
            if user.password == password && user.is_active {
                return Ok(user.clone());
            }
        }
        Err(AuthError::InvalidCredentials)
    }

    pub fn list_users(&self) -> Vec<UserResponse> {
        let users = self.users.read().unwrap();
        users
            .values()
            .map(|user| UserResponse {
                username: user.username.clone(),
                role: user.role.clone(),
                is_active: user.is_active,
            })
            .collect()
    }
}

// === AUTH SERVICE ===
#[derive(Debug, Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthService {
    pub fn new() -> Self {
        dotenvy::dotenv().ok(); // ensure .env is loaded

        let user_repo = UserRepository::new();
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET not found in .env"))
            .trim()
            .to_string();

        Self {
            user_repo,
            encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            jwt_secret,
        }
    }

    pub fn register(&self, user_data: UserCreate) -> Result<UserResponse, AuthError> {
        // Validate role
        if user_data.role != "admin" && user_data.role != "user" {
            return Err(AuthError::InsufficientPermissions);
        }

        let user = User {
            username: user_data.username.clone(),
            password: user_data.password, // In production, hash this
            role: user_data.role.clone(),
            is_active: true,
        };

        self.user_repo.add_user(user)?;

        Ok(UserResponse {
            username: user_data.username,
            role: user_data.role,
            is_active: true,
        })
    }

    pub fn login(&self, login_data: LoginRequest) -> Result<LoginResponse, AuthError> {
        let user = self
            .user_repo
            .validate_credentials(&login_data.username, &login_data.password)?;

        // Generate JWT token
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + (JWT_EXPIRY_HOURS * 3600);

        let claims = Claims {
            sub: user.username.clone(),
            role: user.role.clone(),
            exp: expiration,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(LoginResponse {
            token,
            username: user.username,
            role: user.role,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        // Verify user still exists and is active
        let user = self.user_repo.get_user(&token_data.claims.sub);
        if user.is_none() || !user.unwrap().is_active {
            return Err(AuthError::InvalidToken);
        }

        Ok(token_data.claims)
    }

    pub fn validate_user_role(&self, username: &str, required_role: &str) -> bool {
        if let Some(user) = self.user_repo.get_user(username) {
            user.role == required_role || user.role == "admin"
        } else {
            false
        }
    }

    pub fn get_users(&self) -> Vec<UserResponse> {
        self.user_repo.list_users()
    }
}

// === AUTH MIDDLEWARE EXTRACTOR ===
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthenticatedUser {
    pub username: String,
    pub role: String,
}

impl actix_web::FromRequest for AuthenticatedUser {
    type Error = AuthError;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_service = match req.app_data::<web::Data<AuthService>>() {
            Some(service) => service.clone(),
            None => {
                return Box::pin(async { Err(AuthError::InvalidToken) });
            }
        };

        // Get token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(header[7..].to_string())
                } else {
                    None
                }
            });

        let token = match token {
            Some(token) => token,
            None => {
                return Box::pin(async { Err(AuthError::InvalidToken) });
            }
        };

        Box::pin(async move {
            let claims = auth_service.verify_token(&token)?;
            Ok(AuthenticatedUser {
                username: claims.sub,
                role: claims.role,
            })
        })
    }
}

// === HANDLERS ===

// Health check handler
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy"),
    )
)]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("Auth service is healthy")
}

// Register handler
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = UserCreate,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register(
    user_data: web::Json<UserCreate>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, AuthError> {
    let user = auth_service.register(user_data.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

// Login handler
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(
    login_data: web::Json<LoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<HttpResponse, AuthError> {
    let response = auth_service.login(login_data.into_inner())?;
    Ok(HttpResponse::Ok().json(response))
}

// Profile handler (requires authentication)
#[utoipa::path(
    get,
    path = "/api/v1/auth/profile",
    responses(
        (status = 200, description = "User profile", body = UserProfile),
        (status = 401, description = "Unauthorized"),
    )
)]
async fn get_profile(user: AuthenticatedUser) -> HttpResponse {
    let profile = UserProfile {
        username: user.username,
        role: user.role,
    };
    HttpResponse::Ok().json(profile)
}

// List users handler (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/auth/users",
    responses(
        (status = 200, description = "List of users", body = [UserResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
async fn list_users(
    auth_service: web::Data<AuthService>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AuthError> {
    // Check if user is admin
    if user.role != "admin" {
        return Err(AuthError::InsufficientPermissions);
    }

    let users = auth_service.get_users();
    Ok(HttpResponse::Ok().json(users))
}

// Validate token handler
#[utoipa::path(
    get,
    path = "/api/v1/auth/validate",
    responses(
        (status = 200, description = "Token is valid", body = UserProfile),
        (status = 401, description = "Invalid token"),
    )
)]
async fn validate_token(user: AuthenticatedUser) -> HttpResponse {
    let profile = UserProfile {
        username: user.username,
        role: user.role,
    };
    HttpResponse::Ok().json(profile)
}

// === ROUTE CONFIGURATION ===
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_PREFIX)
            .route("/health", web::get().to(health_check))
            // Public auth routes
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            // Protected auth routes
            .route("/auth/profile", web::get().to(get_profile))
            .route("/auth/validate", web::get().to(validate_token))
            .route("/auth/users", web::get().to(list_users)),
    );
}

// === OPENAPI DOC ===
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        register,
        login,
        get_profile,
        validate_token,
        list_users
    ),
    components(schemas(
        UserCreate, 
        LoginRequest, 
        LoginResponse, 
        UserProfile, 
        UserResponse,
        AuthenticatedUser
    )),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
struct ApiDoc;

// === MAIN ENTRY POINT ===
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(5100);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();

    info!("Starting Auth Service...");

    let auth_service = AuthService::new();

    let address = format!("{}:{}", server_host, server_port);
    info!(
        "🚀 Auth service running on http://{}{}",
        address, API_PREFIX
    );
    info!("Default users: admin/admin123 (admin), user/user123 (user)");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .configure(config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
