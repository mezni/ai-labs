    Files:
        main.rs
        lib.rs
        core/config.rs
        core/logging.rs
        core/errors.rs
    Libraries:
        actix-web
        tokio
        thiserror
        anyhow
        tracing
        tracing-subscriber
    Functionality:
        Actix-web application setup
        CORS (Cross-Origin Resource Sharing) setup
        Application state management in lib.rs
        Logging setup using tracing and tracing-subscriber
        Error handling using thiserror and anyhow
        keep main minimal

give me a modern rust code 

Health Check
    GET /api/v1/health - Check API service status

Registration Flow
    POST /api/v1/register - Initial user registration (submit email/password)
    POST /api/v1/register/verify - Verify email with OTP/token
    POST /api/v1/register/resend - Resend verification code

Authentication
    POST /api/v1/auth/login - User login with credentials
    POST /api/v1/auth/logout - User logout (invalidate session/token)

Recommended Additional Endpoints for Self-Registration:
User Profile Management
    GET /api/v1/users/me - Get current user profile
    PUT /api/v1/users/me - Update user profile
    PATCH /api/v1/users/me/password - Change password


Account Recovery
    POST /api/v1/auth/forgot-password - Request password reset
    POST /api/v1/auth/reset-password - Reset password with token


Session Management
    POST /api/v1/auth/refresh - Refresh access token
    GET /api/v1/auth/sessions - List active sessions
    DELETE /api/v1/auth/sessions/{id} - Revoke specific session

Optional: Account Management
    DELETE /api/v1/users/me - Delete user account (soft/hard delete)
    POST /api/v1/users/me/deactivate - Deactivate account temporarily
    POST /api/v1/register/check-email - Check email availability (pre-registration)





sequenceDiagram
    participant C as Client (Frontend)
    participant A as Auth Service
    participant K as Keycloak
    participant DB as Service Database
    participant J as Job/Cron Service

    %% ====================
    %% REGISTRATION FLOW
    %% ====================
    Note over C,A: 1. REGISTRATION FLOW
    C->>A: POST /api/v1/register
    A->>K: Check if user exists
    K-->>A: User not found
    A->>DB: Create user record (status: "pending_verification")
    DB-->>A: User saved
    A->>K: Create user (enabled: false, emailVerified: false)
    K-->>A: User created in Keycloak
    A->>K: Send verification email
    A->>DB: Set verification expiry (24h)
    A-->>C: 201 Created - Verification email sent

    %% ====================
    %% VERIFICATION FLOWS
    %% ====================
    
    %% Successful verification
    Note over C,A: 2. EMAIL VERIFICATION (SUCCESS)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Still valid (<24h)
    A->>K: Verify email token/OTP
    K-->>A: Email verified
    A->>K: Enable user account
    A->>DB: Update user status to "active"
    A-->>C: 200 OK - Account activated

    %% Expired verification
    Note over C,A: 3. EMAIL VERIFICATION (EXPIRED)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Verification expired (>24h)
    A->>DB: Update status to "verification_expired"
    A->>K: Delete temporary user
    A-->>C: 410 Gone - Verification expired

    %% Resend verification
    Note over C,A: 4. RESEND VERIFICATION
    C->>A: POST /api/v1/register/resend
    A->>DB: Check last sent time & expiry
    DB-->>A: Can resend (within limits)
    A->>K: Resend verification email
    K-->>A: Email sent
    A->>DB: Update resend count & timestamp
    A-->>C: 200 OK - New verification sent

    %% ====================
    %% AUTHENTICATION FLOWS
    %% ====================

    %% Successful login (verified user)
    Note over C,A: 5. LOGIN FLOW (VERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user (OAuth2/OpenID)
    K-->>A: Return tokens (access, refresh, id_token)
    A->>DB: Store session/refresh token
    A-->>C: 200 OK with tokens

    %% Login attempt for unverified user
    Note over C,A: 6. LOGIN FLOW (UNVERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user
    K-->>A: Authentication successful but user disabled
    A->>DB: Check user status
    DB-->>A: Status = "pending_verification"
    A-->>C: 403 Forbidden - Account not verified

    %% ====================
    %% TOKEN MANAGEMENT
    %% ====================

    %% Token validation for API access
    Note over C,A: 7. TOKEN VALIDATION (API ACCESS)
    C->>A: API Request with Bearer token
    A->>K: Introspect token / Validate JWT
    K-->>A: Token valid + user info
    A->>DB: Get user permissions/roles
    A-->>C: Proceed with business logic

    %% Refresh token flow
    Note over C,A: 8. TOKEN REFRESH FLOW
    C->>A: POST /api/v1/auth/refresh
    A->>DB: Validate refresh token
    DB-->>A: Token valid
    A->>K: Refresh token grant
    K-->>A: New access token
    A->>DB: Update session
    A-->>C: New tokens

    %% ====================
    %% SESSION MANAGEMENT
    %% ====================

    %% Logout flow
    Note over C,A: 9. LOGOUT FLOW
    C->>A: POST /api/v1/auth/logout
    A->>K: Revoke tokens
    K-->>A: Tokens revoked
    A->>DB: Invalidate session
    A->>K: Logout user session
    A-->>C: 200 OK

    %% ====================
    %% ALTERNATIVE FLOWS
    %% ====================

    %% Direct Keycloak integration
    Note over C,K: 10. DIRECT KEYCLOAK FLOW (Alternative)
    C->>K: Redirect to Keycloak login
    K->>C: Return authorization code
    C->>A: Send code for token exchange
    A->>K: Exchange code for tokens
    K-->>A: Return tokens
    A-->>C: Tokens for API access

    %% ====================
    %% MAINTENANCE FLOWS
    %% ====================

    %% Cleanup job for expired registrations
    Note over J,A: 11. CLEANUP PROCESS (DAILY JOB)
    J->>DB: Find expired pending verifications
    DB-->>J: List of expired users
    loop For each expired user
        J->>K: Delete user from Keycloak
        J->>DB: Mark as "cleaned_up"
    end

    %% Re-registration after expiry
    Note over C,A: 12. RE-REGISTRATION AFTER EXPIRY
    C->>A: POST /api/v1/register (same email)
    A->>DB: Check previous records
    DB-->>A: Previous status = "verification_expired"
    A->>K: Check if user exists
    K-->>A: User not found (was deleted)
    A->>DB: Create new registration record
    A->>K: Create new temporary user
    A-->>C: 201 Created - New verification sent



classDiagram
    %% ====================
    %% ENUMERATIONS
    %% ====================
    class KeycloakError {
        <<enumeration>>
        +Authentication(String)
        +TokenValidation(String)
        +UserManagement(String)
        +Network(String)
        +Config(String)
        +display(): String
    }

    %% ====================
    %% CONFIGURATION
    %% ====================
    class KeycloakConfig {
        -keycloak_url: String
        -keycloak_realm: String
        -auth_client_id: String
        -auth_client_redirect_uri: String
        -backend_client_id: String
        -backend_client_secret: String
        -token_expiry_buffer_seconds: i64
        -admin_token_refresh_interval: u64
        +from_env() KeycloakConfig
        +get_keycloak_url() String
        +get_keycloak_realm() String
        +get_auth_client_id() String
        +get_backend_client_id() String
        +get_backend_client_secret() String
    }

    %% ====================
    %% DATA STRUCTURES
    %% ====================
    class AdminToken {
        -access_token: String
        -refresh_token: Option~String~
        -expires_at: DateTime~Utc~
        -token_type: String
        -scope: Option~String~
        +is_expired() bool
        +get_access_token() String
        +get_refresh_token() Option~String~
    }

    class UserInfo {
        +sub: String
        +email: String
        +email_verified: bool
        +preferred_username: Option~String~
        +given_name: Option~String~
        +family_name: Option~String~
        +enabled: bool
        +is_active() bool
    }

    class CreateUserRequest {
        +username: String
        +email: String
        +enabled: bool
        +email_verified: bool
        +credentials: Vec~Credential~
        +attributes: Option~serde_json::Value~
        +new(username, email, password) CreateUserRequest
        +add_attribute(key, value)
    }

    class Credential {
        +type: String
        +value: String
        +temporary: bool
        +new_password(value) Credential
        +new_totp(value) Credential
    }

    class TokenResponse {
        <<serializable>>
        +access_token: String
        +refresh_token: Option~String~
        +expires_in: i64
        +refresh_expires_in: i64
        +token_type: String
        +scope: Option~String~
        +id_token: Option~String~
    }

    %% ====================
    %% MAIN CLIENT CLASS
    %% ====================
    class KeycloakClient {
        -http: reqwest::Client
        -config: KeycloakConfig
        -admin_token: Arc~RwLock~Option~AdminToken~~~
        +new(config: KeycloakConfig) KeycloakClient
        +with_custom_http(client: reqwest::Client) KeycloakClient
    }

    %% ====================
    %% TRAIT INTERFACES
    %% ====================
    class IAdminOperations {
        <<interface>>
        +create_user(user: CreateUserRequest) Result~String, KeycloakError~
        +get_user_by_email(email: &str) Result~Option~UserInfo~, KeycloakError~
        +update_user(user_id: &str, update: UserUpdate) Result~(), KeycloakError~
        +delete_user(user_id: &str) Result~(), KeycloakError~
        +execute_email_verification(user_id: &str) Result~(), KeycloakError~
        +send_actions_email(user_id: &str, actions: Vec~String~) Result~(), KeycloakError~
        +list_users(query: UserQuery) Result~Vec~UserInfo~, KeycloakError~
    }

    class IAuthentication {
        <<interface>>
        +authenticate_user(username: &str, password: &str) Result~TokenResponse, KeycloakError~
        +validate_token(token: &str) Result~UserInfo, KeycloakError~
        +refresh_token(refresh_token: &str) Result~TokenResponse, KeycloakError~
        +logout(refresh_token: &str) Result~(), KeycloakError~
        +introspect_token(token: &str) Result~TokenInfo, KeycloakError~
    }

    class ITokenManager {
        <<interface>>
        +get_admin_token() Result~String, KeycloakError~
        +fetch_admin_token() Result~AdminToken, KeycloakError~
        +clear_admin_token()
        +is_admin_token_valid() bool
        +schedule_token_refresh()
    }

    %% ====================
    %% SUPPORTING CLASSES
    %% ====================
    class UserQuery {
        +email: Option~String~
        +username: Option~String~
        +first_name: Option~String~
        +last_name: Option~String~
        +enabled: Option~bool~
        +email_verified: Option~bool~
        +first: Option~i32~
        +max: Option~i32~
        +to_query_params() Vec~(String, String)~
    }

    class UserUpdate {
        +email: Option~String~
        +enabled: Option~bool~
        +email_verified: Option~bool~
        +attributes: Option~serde_json::Value~
        +to_json() serde_json::Value
    }

    class TokenInfo {
        +active: bool
        +sub: String
        +username: String
        +email: String
        +email_verified: bool
        +realm_access: serde_json::Value
        +resource_access: serde_json::Value
        +exp: i64
        +iat: i64
        +is_expired() bool
    }

    class EmailAction {
        <<enumeration>>
        +VERIFY_EMAIL
        +UPDATE_PASSWORD
        +UPDATE_PROFILE
        +CONFIGURE_TOTP
        +to_string() String
    }

    %% ====================
    %% BUILDER PATTERN
    %% ====================
    class KeycloakClientBuilder {
        -config: Option~KeycloakConfig~
        -http_client: Option~reqwest::Client~
        +new() KeycloakClientBuilder
        +with_config(config: KeycloakConfig) KeycloakClientBuilder
        +with_env_config() KeycloakClientBuilder
        +with_http_client(client: reqwest::Client) KeycloakClientBuilder
        +with_timeout(timeout: Duration) KeycloakClientBuilder
        +build() Result~KeycloakClient, KeycloakError~
    }

    %% ====================
    %% FACTORY
    %% ====================
    class KeycloakClientFactory {
        -instances: HashMap~String, Arc~KeycloakClient~~
        +get_instance(realm: &str) Result~Arc~KeycloakClient~, KeycloakError~
        +create_client(config: KeycloakConfig) Result~Arc~KeycloakClient~, KeycloakError~
        +clear_instance(realm: &str)
        +clear_all()
    }

    %% ====================
    %% RELATIONSHIPS
    %% ====================
    KeycloakClient --> KeycloakConfig : uses
    KeycloakClient --> AdminToken : manages
    KeycloakClient --> IAdminOperations : implements
    KeycloakClient --> IAuthentication : implements
    KeycloakClient --> ITokenManager : implements
    
    CreateUserRequest --> Credential : contains
    KeycloakClientBuilder --> KeycloakClient : builds
    KeycloakClientFactory --> KeycloakClient : creates/manages
    
    UserQuery --> KeycloakClient : used by
    UserUpdate --> KeycloakClient : used by
    
    KeycloakError <|-- KeycloakClient : raises
    
    %% Composition relationships
    KeycloakClient "1" *-- "1" KeycloakConfig
    KeycloakClient "1" *-- "1" reqwest::Client
    KeycloakClient "1" *-- "0..1" AdminToken
    
    %% Aggregation relationships
    CreateUserRequest "1" *-- "*" Credential
    
    %% Dependency relationships
    IAdminOperations ..> CreateUserRequest
    IAdminOperations ..> UserInfo
    IAuthentication ..> TokenResponse
    ITokenManager ..> AdminToken    





classDiagram
    %% Main Application Entry Point
    class App {
        +main() async
        +configure_services() AppState
        +configure_routes() HttpServer
    }
    
    %% Core Configuration
    class AppConfig {
        +database: DatabaseConfig
        +keycloak: KeycloakConfig
        +server: ServerConfig
        +auth: AuthConfig
        +smtp: Option~SmtpConfig~
        +rate_limit: RateLimitConfig
        +log_level: String
        +new() Result~Self, ConfigError~
    }
    
    class AppState {
        -db: PgPool
        -config: AppConfig
        -keycloak_client: Arc~dyn KeycloakClient~
        -user_registration_repo: Arc~dyn UserRegistrationRepository~
        -user_repo: Arc~dyn UserRepository~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        -email_service: Arc~dyn EmailService~
    }
    
    %% Domain Layer
    class User {
        -id: Uuid
        -keycloak_id: String
        -email: String
        -username: Option~String~
        -roles: Vec~String~
        -status: String
        +validate() Result~
    }
    
    class UserRegistration {
        -id: Uuid
        -email: String
        -verification_token: String
        -verification_token_expires_at: DateTime~Utc~
        -keycloak_sync_status: String
        +is_expired() bool
        +is_verified() bool
    }
    
    %% Domain Interfaces (Traits)
    <<interface>> UserRepository
    UserRepository : +create(user: &User) AppResult~User~
    UserRepository : +find_by_id(id: Uuid) AppResult~Option~User>~
    UserRepository : +find_by_email(email: &str) AppResult~Option~User>~
    UserRepository : +update(user: &User) AppResult~User~
    UserRepository : +delete(id: Uuid) AppResult~()~
    
    <<interface>> UserRegistrationRepository
    UserRegistrationRepository : +create(reg: &UserRegistration) AppResult~UserRegistration~
    UserRegistrationRepository : +find_by_token(token: &str) AppResult~Option~UserRegistration>~
    UserRegistrationRepository : +find_by_email(email: &str) AppResult~Option~UserRegistration>~
    UserRegistrationRepository : +update(reg: &UserRegistration) AppResult~UserRegistration~
    UserRegistrationRepository : +delete_expired(before: DateTime~Utc~) AppResult~u64~
    
    <<interface>> KeycloakClient
    KeycloakClient : +create_user(email, username, first_name, last_name, password) AppResult~String~ // Returns Keycloak user ID
    KeycloakClient : +enable_user(keycloak_id) AppResult~()~
    KeycloakClient : +disable_user(keycloak_id) AppResult~()~
    KeycloakClient : +assign_role(keycloak_id, role) AppResult~()~
    KeycloakClient : +verify_token(token) AppResult~serde_json::Value~ // Returns token claims
    KeycloakClient : +get_user_info(token) AppResult~serde_json::Value~ // Returns user info
    
    <<interface>> EmailService
    EmailService : +send_verification_email(to: String, token: String) AppResult~()~
    EmailService : +send_password_reset(to: String, token: String) AppResult~()~
    EmailService : +send_welcome_email(to: String, username: String) AppResult~()~
    
    %% Application Services Layer
    class UserRegistrationService {
        -user_registration_repo: Arc~dyn UserRegistrationRepository~
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -email_service: Arc~dyn EmailService~
        -config: AppConfig
        +register_user(command: RegisterUserCommand) AppResult~UserRegistration~
        +verify_registration(command: VerifyRegistrationCommand) AppResult~User~
        +resend_verification(email: String) AppResult~()~
    }
    
    class UserService {
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        +get_user(id: Uuid) AppResult~User~
        +update_profile(id: Uuid, command: UpdateProfileCommand) AppResult~User~
        +change_password(id: Uuid, command: ChangePasswordCommand) AppResult~()~
        +disable_account(id: Uuid) AppResult~()~
        +list_users(query: UserQuery) AppResult~Vec~User>~
    }
    
    class AuthService {
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        +authenticate(command: LoginCommand) AppResult~AuthResponse~
        +refresh_token(refresh_token: String) AppResult~AuthResponse~
        +logout(access_token: String) AppResult~()~
        +validate_token(token: String) AppResult~TokenClaims~
    }
    
    %% Infrastructure Layer - Repositories
    class PostgresUserRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
        +with_transaction(tx, user) AppResult~User~
    }
    
    class PostgresUserRegistrationRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
    }
    
    class PostgresLoginAuditRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
    }
    
    class KeycloakClientImpl {
        -http_client: reqwest::Client
        -config: KeycloakConfig
        +get_admin_token() AppResult~String~
        +create_user() AppResult~String~
        +enable_user() AppResult~()~
        +verify_token() AppResult~serde_json::Value~
    }
    
    class SmtpEmailService {
        -config: SmtpConfig
        -client: lettre::AsyncSmtpTransport
        +send_email(email: Email) AppResult~()~
    }
    
    class MockEmailService {
        -sent_emails: Vec~Email~
        +get_sent_emails() Vec~Email~
    }
    
    %% HTTP Layer (Controllers/Handlers)
    class RegistrationHandler {
        -service: Arc~UserRegistrationService~
        +register(request: HttpRequest, payload: web::Json~RegisterRequest~) HttpResponse
        +verify(request: HttpRequest, payload: web::Json~VerifyRequest~) HttpResponse
        +resend_verification(request: HttpRequest, payload: web::Json~ResendRequest~) HttpResponse
    }
    
    class AuthHandler {
        -service: Arc~AuthService~
        +login(request: HttpRequest, payload: web::Json~LoginRequest~) HttpResponse
        +refresh(request: HttpRequest, payload: web::Json~RefreshRequest~) HttpResponse
        +logout(request: HttpRequest) HttpResponse
        +validate(request: HttpRequest) HttpResponse
    }
    
    class UserHandler {
        -service: Arc~UserService~
        +get_profile(request: HttpRequest) HttpResponse
        +update_profile(request: HttpRequest, payload: web::Json~UpdateProfileRequest~) HttpResponse
        +change_password(request: HttpRequest, payload: web::Json~ChangePasswordRequest~) HttpResponse
    }
    
    class AdminHandler {
        -service: Arc~UserService~
        +list_users(request: HttpRequest) HttpResponse
        +get_user(request: HttpRequest) HttpResponse
        +disable_user(request: HttpRequest) HttpResponse
        +enable_user(request: HttpRequest) HttpResponse
    }
    
    %% DTOs (Data Transfer Objects)
    class RegisterUserCommand {
        -email: String
        -username: Option~String~
        -password: String
        -confirm_password: String
        -first_name: Option~String~
        -last_name: Option~String~
        -source: String
        +validate() Result~
    }
    
    class RegisterRequest {
        -email: String
        -username: Option~String~
        -password: String
        -confirm_password: String
        -first_name: Option~String~
        -last_name: Option~String~
    }
    
    class RegisterResponse {
        -registration_id: Uuid
        -email: String
        -expires_at: DateTime~Utc~
        -message: String
    }
    
    class VerifyRequest {
        -token: String
    }
    
    class LoginRequest {
        -email: String
        -password: String
        -device_id: Option~String~
    }
    
    class AuthResponse {
        -access_token: String
        -refresh_token: String
        -expires_in: i64
        -token_type: String
        -user: UserDto
    }
    
    class UserDto {
        -id: Uuid
        -email: String
        -username: Option~String~
        -first_name: Option~String~
        -last_name: Option~String~
        -roles: Vec~String~
    }
    
    %% Middleware
    class AuthMiddleware {
        -keycloak_client: Arc~dyn KeycloakClient~
        +extract_token(request: &HttpRequest) Option~String~
        +validate_token(token: String) AppResult~TokenClaims~
    }
    
    class RateLimitMiddleware {
        -redis_client: redis::Client
        -config: RateLimitConfig
        +check_limit(key: String) AppResult~bool~
        +increment_counter(key: String) AppResult~()~
    }
    
    class LoggingMiddleware {
        +log_request(request: &ServiceRequest)
        +log_response(response: &ServiceResponse)
    }
    
    %% Error Types
    class AppError {
        -kind: ErrorKind
        -message: String
        +validation(msg: String) Self
        +authentication(msg: String) Self
        +not_found(msg: String) Self
        +conflict(msg: String) Self
        +database(err: sqlx::Error) Self
        +keycloak(msg: String) Self
        +error_response() HttpResponse
    }
    
    class ErrorKind {
        <<enumeration>>
        Validation
        Authentication
        Authorization
        NotFound
        Conflict
        Database
        Keycloak
        ExternalService
        Internal
    }
    
    %% Relationships
    App --> AppConfig : creates
    App --> AppState : configures
    App --> HttpServer : starts
    
    AppState o-- "1" AppConfig : contains
    AppState o-- "1" PgPool : manages
    AppState o-- "*" dyn Repository : holds
    
    UserRegistrationService --> "1" UserRegistrationRepository : uses
    UserRegistrationService --> "1" UserRepository : uses
    UserRegistrationService --> "1" KeycloakClient : uses
    UserRegistrationService --> "1" EmailService : uses
    
    PostgresUserRepository ..|> UserRepository : implements
    PostgresUserRegistrationRepository ..|> UserRegistrationRepository : implements
    KeycloakClientImpl ..|> KeycloakClient : implements
    SmtpEmailService ..|> EmailService : implements
    
    RegistrationHandler --> "1" UserRegistrationService : depends on
    AuthHandler --> "1" AuthService : depends on
    UserHandler --> "1" UserService : depends on
    
    AuthMiddleware --> "1" KeycloakClient : uses
    
    RegisterRequest --> RegisterUserCommand : maps to
    User --> UserDto : converts to
    
    RegistrationHandler --> RegisterRequest : accepts
    RegistrationHandler --> RegisterResponse : returns









