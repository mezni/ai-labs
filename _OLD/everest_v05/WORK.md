Health, Registration, Authentication, Password Management, User Profile, Admin, and Invitations,



HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgresql://postgres:password@localhost:5800/auth_db
# Options: error, warn, info, debug, trace
RUST_LOG=debug


# Keycloak Configuration
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_REALM=myrealm

# Frontend client (public, for user authentication)
KEYCLOAK_AUTH_CLIENT_ID=auth-client

# Backend service account (confidential, for admin operations)
KEYCLOAK_BACKEND_CLIENT_ID=backend-admin
KEYCLOAK_BACKEND_CLIENT_SECRET=backend-admin-secret


actix-web = "4.12.1"
anyhow = "1.0.100"
async-trait = "0.1.89"
chrono = { version = "0.4.42", features = ["serde"] }
dotenvy = "0.15.7"
nanoid = "0.4.0"
regex = "1.12.2"
reqwest = { version = "0.12.26", features = ["json"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
sqlx = { version = "0.8.6", features = ["chrono", "macros", "postgres", "runtime-tokio-rustls"] }
thiserror = "2.0.17"
tokio = { version = "1.48.0", features = ["full"] }
tracing = "0.1.44"
tracing-actix-web = "0.7.20"
tracing-subscriber = { version = "0.3.22", features = ["env-filter"] }
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }


sequenceDiagram
    actor Client as Client (Frontend/Mobile)
    participant API as Auth Service API
    participant RegS as RegistrationService
    participant AuthS as AuthenticationService
    participant DB as Postgres DB
    participant KC as Keycloak
    participant Email as Email/SMS

    %% ====================
    %% REGISTRATION FLOW
    %% ====================
    Note over Client,API: 1. Registration
    Client->>API: POST /api/v1/registration/register
    API->>RegS: register_user(email, username, password, ...)
    RegS->>DB: check existing registration/email
    DB-->>RegS: exists? false
    RegS->>DB: insert into user_registrations
    RegS->>Email: send verification email/SMS
    Email-->>Client: receives email/SMS
    RegS-->>API: registration created
    API-->>Client: 201 Created (registration_id, expires_at)

    %% ====================
    %% VERIFICATION FLOW
    %% ====================
    Note over Client,API: 2. Verify registration
    Client->>API: POST /api/v1/registration/verify {token/code}
    API->>RegS: verify_registration(token/code)
    RegS->>DB: get registration by token
    DB-->>RegS: registration found
    RegS->>DB: update status to verified
    RegS->>KC: create user in Keycloak
    KC-->>RegS: user_id
    RegS->>DB: update user_id in registration
    RegS-->>API: verification successful
    API-->>Client: 200 OK

    %% ====================
    %% RESEND VERIFICATION FLOW
    %% ====================
    Note over Client,API: 3. Resend verification
    Client->>API: POST /api/v1/registration/verify/resend
    API->>RegS: resend_verification(email)
    RegS->>DB: get pending registration by email
    DB-->>RegS: registration found
    RegS->>Email: resend verification email/SMS
    RegS-->>API: success
    API-->>Client: 200 OK

    %% ====================
    %% LOGIN FLOW
    %% ====================
    Note over Client,API: 4. Authentication
    Client->>API: POST /api/v1/auth/login
    API->>AuthS: login_user(email/username, password)
    AuthS->>KC: validate credentials
    KC-->>AuthS: user authenticated
    AuthS->>DB: update last_login, session
    AuthS->>API: generate JWT access + refresh token
    API-->>Client: 200 OK {access_token, refresh_token}

    %% ====================
    %% TOKEN REFRESH
    %% ====================
    Client->>API: POST /api/v1/auth/refresh {refresh_token}
    API->>AuthS: refresh_token(refresh_token)
    AuthS->>DB: validate refresh_token
    AuthS->>API: issue new access_token
    API-->>Client: 200 OK {new_access_token}

    %% ====================
    %% LOGOUT
    %% ====================
    Client->>API: POST /api/v1/auth/logout
    API->>AuthS: logout_user(session_id)
    AuthS->>DB: invalidate refresh_token
    AuthS-->>API: logout complete
    API-->>Client: 200 OK


sequenceDiagram
    actor Admin as Admin Client
    actor User as User Client
    participant API as Auth Service API
    participant AdminS as AdminService
    participant InvS as InvitationService
    participant DB as Postgres DB
    participant KC as Keycloak
    participant Email as Email/SMS

    %% ====================
    %% ADMIN USER MANAGEMENT
    %% ====================
    Note over Admin,API: 1. List all users
    Admin->>API: GET /api/v1/admin/users
    API->>AdminS: list_users()
    AdminS->>DB: fetch all users
    DB-->>AdminS: users list
    AdminS-->>API: return users
    API-->>Admin: 200 OK

    Note over Admin,API: 2. Get user by ID
    Admin->>API: GET /api/v1/admin/users/{id}
    API->>AdminS: get_user_by_id(id)
    AdminS->>DB: fetch user
    DB-->>AdminS: user
    AdminS-->>API: return user
    API-->>Admin: 200 OK

    Note over Admin,API: 3. Create user
    Admin->>API: POST /api/v1/admin/users
    API->>AdminS: create_user(user_data)
    AdminS->>DB: insert user
    AdminS->>KC: create Keycloak user
    KC-->>AdminS: user_id
    AdminS->>DB: update Keycloak id
    AdminS-->>API: user created
    API-->>Admin: 201 Created

    Note over Admin,API: 4. Update user
    Admin->>API: PUT /api/v1/admin/users/{id}
    API->>AdminS: update_user(id, data)
    AdminS->>DB: update user record
    AdminS->>KC: update Keycloak user
    AdminS-->>API: user updated
    API-->>Admin: 200 OK

    Note over Admin,API: 5. Delete user
    Admin->>API: DELETE /api/v1/admin/users/{id}
    API->>AdminS: delete_user(id)
    AdminS->>DB: soft delete / mark deleted
    AdminS->>KC: disable Keycloak user
    AdminS-->>API: user deleted
    API-->>Admin: 200 OK

    %% ====================
    %% INVITATIONS
    %% ====================
    Note over Admin,API: 6. Create invitation
    Admin->>API: POST /api/v1/invitations
    API->>InvS: create_invitation(data)
    InvS->>DB: insert invitation
    InvS->>Email: send invitation email
    InvS-->>API: invitation created
    API-->>Admin: 201 Created

    Note over Admin,API: 7. List invitations
    Admin->>API: GET /api/v1/invitations
    API->>InvS: list_invitations()
    InvS->>DB: fetch invitations
    DB-->>InvS: invitations list
    InvS-->>API: return invitations
    API-->>Admin: 200 OK

    Note over User,API: 8. Get invitation by code
    User->>API: GET /api/v1/invitations/{code}
    API->>InvS: get_invitation(code)
    InvS->>DB: fetch invitation
    DB-->>InvS: invitation data
    InvS-->>API: return invitation
    API-->>User: 200 OK

    Note over User,API: 9. Accept invitation
    User->>API: POST /api/v1/invitations/{code}/accept
    API->>InvS: accept_invitation(code, user_data)
    InvS->>DB: update invitation status
    InvS->>AdminS: create user (via RegistrationService)
    InvS-->>API: invitation accepted
    API-->>User: 200 OK

    Note over Admin,API: 10. Cancel invitation
    Admin->>API: DELETE /api/v1/invitations/{code}
    API->>InvS: cancel_invitation(code)
    InvS->>DB: mark invitation cancelled
    InvS-->>API: invitation cancelled
    API-->>Admin: 200 OK



classDiagram
%% ====================
%% DOMAIN ENTITIES
%% ====================
class User {
    +user_id: String
    +keycloak_id: Option<String>
    +email: String
    +username: String
    +first_name: Option<String>
    +last_name: Option<String>
    +phone: Option<String>
    +role: Role
    +status: UserStatus
    +source: Source
    +created_at: DateTime
    +updated_at: DateTime
    +last_login_at: Option<DateTime>
    +avatar_url: Option<String>
    +gdpr_anonymized_at: Option<DateTime>
    +login_count: Int
    +accept_terms_at: Option<DateTime>
    +accept_privacy_at: Option<DateTime>
    +marketing_consent: Bool
    +data_processing_consent: Bool
}

class Registration {
    +registration_id: String
    +email: String
    +username: String
    +first_name: Option<String>
    +last_name: Option<String>
    +phone: Option<String>
    +verification_token: Option<String>
    +verification_code: Option<String>
    +status: RegistrationStatus
    +expires_at: DateTime
    +resend_count: Int
    +source: Source
    +created_at: DateTime
    +updated_at: DateTime
}

class Invitation {
    +code: String
    +email: String
    +status: InvitationStatus
    +expires_at: DateTime
    +created_by: String
    +created_at: DateTime
}

%% ====================
%% ENUMS / VALUE OBJECTS
%% ====================
class Role {
    <<enumeration>>
    user
    admin
    partner
    operator
}

class UserStatus {
    <<enumeration>>
    pending
    active
    inactive
    locked
    suspended
    deleted
}

class RegistrationStatus {
    <<enumeration>>
    pending
    verified
    expired
    cancelled
}

class Source {
    <<enumeration>>
    web
    mobile
    api
    admin
    import
    sso
}

class InvitationStatus {
    <<enumeration>>
    pending
    accepted
    cancelled
    expired
}

%% ====================
%% REPOSITORIES
%% ====================
class UserRepository {
    +get_by_id(user_id: String)
    +get_all()
    +create(user: User)
    +update(user: User)
    +delete(user_id: String)
}

class RegistrationRepository {
    +get_by_id(registration_id: String)
    +create(reg: Registration)
    +update(reg: Registration)
    +exists_by_email(email: String)
    +get_by_token(token: String)
}

class InvitationRepository {
    +get_by_code(code: String)
    +create(inv: Invitation)
    +update(inv: Invitation)
    +delete(code: String)
    +list_all()
}

class RefreshTokenRepository {
    +create(token: String, user_id: String)
    +validate(token: String)
    +revoke(token: String)
}

%% ====================
%% SERVICES
%% ====================
class RegistrationService {
    +register_user(...)
    +verify_registration(token)
    +resend_verification(email)
}

class AuthenticationService {
    +login_user(email, password)
    +logout_user(session_id)
    +refresh_token(refresh_token)
    +validate_token(token)
}

class AdminService {
    +list_users()
    +get_user_by_id(user_id)
    +create_user(user_data)
    +update_user(user_id, user_data)
    +delete_user(user_id)
}

class InvitationService {
    +create_invitation(data)
    +list_invitations()
    +get_invitation(code)
    +accept_invitation(code, user_data)
    +cancel_invitation(code)
}

class EmailSender {
    +send_verification_email(email, token, code)
    +send_invitation_email(email, code)
}

%% ====================
%% RELATIONSHIPS
%% ====================
UserRepository <|.. User
RegistrationRepository <|.. Registration
InvitationRepository <|.. Invitation
RefreshTokenRepository <|.. AuthenticationService

RegistrationService --> RegistrationRepository
RegistrationService --> EmailSender
AuthenticationService --> UserRepository
AuthenticationService --> RefreshTokenRepository
AdminService --> UserRepository
InvitationService --> InvitationRepository
InvitationService --> RegistrationService
InvitationService --> EmailSender



classDiagram
%% ====================
%% DOMAIN ENTITIES
%% ====================
class User {
    +user_id: String
    +keycloak_id: Option<String>
    +email: String
    +username: String
    +first_name: Option<String>
    +last_name: Option<String>
    +phone: Option<String>
    +role: Role
    +status: UserStatus
    +source: Source
    +created_at: DateTime
    +updated_at: DateTime
    +last_login_at: Option<DateTime>
    +avatar_url: Option<String>
    +gdpr_anonymized_at: Option<DateTime>
    +login_count: Int
    +accept_terms_at: Option<DateTime>
    +accept_privacy_at: Option<DateTime>
    +marketing_consent: Bool
    +data_processing_consent: Bool
}

class Registration {
    +registration_id: String
    +email: String
    +username: String
    +first_name: Option<String>
    +last_name: Option<String>
    +phone: Option<String>
    +verification_token: Option<String>
    +verification_code: Option<String>
    +status: RegistrationStatus
    +expires_at: DateTime
    +resend_count: Int
    +source: Source
    +created_at: DateTime
    +updated_at: DateTime
}

class Invitation {
    +code: String
    +email: String
    +status: InvitationStatus
    +expires_at: DateTime
    +created_by: String
    +created_at: DateTime
}

%% ====================
%% ENUMS
%% ====================
class Role {
    <<enumeration>>
    user
    admin
    partner
    operator
}

class UserStatus {
    <<enumeration>>
    pending
    active
    inactive
    locked
    suspended
    deleted
}

class RegistrationStatus {
    <<enumeration>>
    pending
    verified
    expired
    cancelled
}

class Source {
    <<enumeration>>
    web
    mobile
    api
    admin
    import
    sso
}

class InvitationStatus {
    <<enumeration>>
    pending
    accepted
    cancelled
    expired
}

%% ====================
%% REPOSITORIES
%% ====================
class UserRepository {
    +get_by_id(user_id)
    +get_all()
    +create(user)
    +update(user)
    +delete(user_id)
}

class RegistrationRepository {
    +get_by_id(registration_id)
    +create(reg)
    +update(reg)
    +exists_by_email(email)
    +get_by_token(token)
}

class InvitationRepository {
    +get_by_code(code)
    +create(inv)
    +update(inv)
    +delete(code)
    +list_all()
}

class RefreshTokenRepository {
    +create(token, user_id)
    +validate(token)
    +revoke(token)
}

%% ====================
%% SERVICES
%% ====================
class RegistrationService {
    +register_user(...)
    +verify_registration(token)
    +resend_verification(email)
}

class AuthenticationService {
    +login_user(email, password)
    +logout_user(session_id)
    +refresh_token(refresh_token)
    +validate_token(token)
}

class AdminService {
    +list_users()
    +get_user_by_id(user_id)
    +create_user(user_data)
    +update_user(user_id, user_data)
    +delete_user(user_id)
}

class InvitationService {
    +create_invitation(data)
    +list_invitations()
    +get_invitation(code)
    +accept_invitation(code, user_data)
    +cancel_invitation(code)
}

class EmailSender {
    +send_verification_email(email, token, code)
    +send_invitation_email(email, code)
}

%% ====================
%% CONTROLLERS
%% ====================
class RegistrationController {
    +POST /register
    +POST /verify
    +POST /verify/resend
}

class AuthenticationController {
    +POST /auth/login
    +POST /auth/logout
    +POST /auth/refresh
    +POST /auth/validate
}

class HealthController {
    +GET /health
}

class AdminController {
    +GET /admin/users
    +GET /admin/users/id
    +POST /admin/users
    +PUT /admin/users/id
    +DELETE /admin/users/id
}

class InvitationController {
    +POST /invitations
    +GET /invitations
    +GET /invitations/<code>
    +POST /invitations/<code>/accept
    +DELETE /invitations/<code>
}

%% ====================
%% RELATIONSHIPS
%% ====================
UserRepository <|.. User
RegistrationRepository <|.. Registration
InvitationRepository <|.. Invitation

RegistrationService --> RegistrationRepository
RegistrationService --> EmailSender

AuthenticationService --> UserRepository
AuthenticationService --> RefreshTokenRepository

AdminService --> UserRepository
InvitationService --> InvitationRepository
InvitationService --> RegistrationService
InvitationService --> EmailSender

RegistrationController --> RegistrationService
AuthenticationController --> AuthenticationService
AdminController --> AdminService
InvitationController --> InvitationService
HealthController --> HealthService




src/
├── lib.rs                  # Public API re-exports (if library crate)
├── main.rs                 # Thin entry point
│
├── core/                   # Shared core utilities
│   ├── config.rs
│   ├── logging.rs
│   ├── error.rs            # Centralized error types
│   ├─ constants.rs
│   └── mod.rs
    ├── password.rs
    ├── id_generator.rs
    ├── jwt.rs

├── domain/                 # Pure domain logic (no external deps)
│   ├── entities.rs
│   ├── enums.rs
│   ├── value_objects.rs
│   ├── repositories.rs     # Repository traits
│   ├── services.rs         # Domain service traits
│   └── mod.rs
│
├── application/            # Use cases & application services
│   ├── mod.rs
│   ├── dto/                # All Data Transfer Objects grouped
│   │   ├── registration.rs
│   │   ├── authentication.rs
│   │   ├── health.rs
│   │   └── mod.rs
│   ├── registration_service.rs
│   ├── authentication_service.rs
│   ├── admin_service.rs
│   ├── invitation_service.rs
│   └── health_service.rs
│
├── infrastructure/        # External implementations
│   ├── mod.rs
│   ├── keycloak_client.rs
│   └── repositories/       # Concrete repository impls
│       ├── user_repository.rs
│       ├── registration_repository.rs
│       ├── invitation_repository.rs
│       └── mod.rs
│
├── presentation/           # HTTP layer
│   ├── mod.rs
│   ├── controllers/
│   │   ├── registration_controller.rs
│   │   ├── authentication_controller.rs
│   │   ├── admin_controller.rs
│   │   ├── invitation_controller.rs
│   │   ├── health_controller.rs
│   │   └── mod.rs
│   ├── middleware/         # Auth, logging, error handling, etc.
│   │   ├── auth.rs
│   │   └── mod.rs
│   └── openapi.rs          # OpenAPI spec
