Auth Service is the only interface to Keycloak.
Keycloak handles all emails — verification, invitation, password reset.
Database tracks local metadata, audit logs, and preferences.
Supports both public self-registration and admin-driven user creation/invitation.
Role updates and soft deletes are fully logged and synchronized with Keycloak.


Keycloak is fully encapsulated behind the Auth Service.
Database is for metadata, audit, preferences, and synchronization logs only.
Clean separation:
User actions: registration, verification, login
Admin actions: invite, create, role change, deactivate
Ideal for documentation, ADRs, and architecture slides because it’s readable and compact.

Clear separation of responsibilities:
User: registration, verification, login
Admin: invitations, role changes, deactivation
Auth Service: orchestrates all actions, only component talking to Keycloak
Keycloak: handles authentication, email sending, role assignment
Database: stores metadata, audit logs, preferences, and sync logs
Single source of truth for identity: Keycloak
Centralized audit & synchronization: via database
No external email service; all emails go through Keycloak


sequenceDiagram
    autonumber
    actor User
    actor Admin
    participant API as Auth Service
    participant DB as PostgreSQL
    participant KC as Keycloak

    %% ==========================
    %% Self-Registered User Flow
    %% ==========================
    User->>API: Submit registration data (email, username, profile)
    API->>DB: INSERT user_registrations (pending)
    API->>KC: Create user (role=user, send verification email)
    KC-->>API: keycloak_id
    API->>DB: INSERT keycloak_sync_log (create, success)
    API->>DB: INSERT users (is_verified=false, source=web)
    API->>DB: INSERT user_preferences

    User->>KC: Click verification link in email
    KC-->>API: Verification confirmation
    API->>DB: UPDATE user_registrations status=verified
    API->>DB: UPDATE users is_verified=true
    API->>DB: INSERT email_verification_logs

    User->>API: Login with credentials
    API->>KC: Authenticate user
    KC-->>API: JWT tokens
    API-->>User: Return JWT
    API->>DB: INSERT login_audit_log (success=true)

    %% ==========================
    %% Admin Invitation Flow
    %% ==========================
    Admin->>API: Create invitation (email, role, network_id, station_id)
    API->>DB: INSERT user_invitations (pending)
    API->>KC: Create user (role, send welcome/setup email)
    KC-->>API: keycloak_id
    API->>DB: INSERT keycloak_sync_log (create, success)
    API->>DB: INSERT users (source=internal, is_verified=true)
    API->>DB: INSERT user_preferences

    User->>KC: Accept invitation via email link
    KC-->>API: Confirmation
    API->>DB: UPDATE user_invitations status=accepted

    User->>API: Login with credentials
    API->>KC: Authenticate user
    KC-->>API: JWT tokens
    API-->>User: Return JWT
    API->>DB: INSERT login_audit_log (success=true)

    %% ==========================
    %% Admin Role Change / Deactivation
    %% ==========================
    Admin->>API: Change role / deactivate user
    API->>KC: Update user (role or enabled=false)
    KC-->>API: Success
    API->>DB: UPDATE users
    API->>DB: INSERT keycloak_sync_log (role_update/status_update)






auth-service/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   └── mod.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── aggregates/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── user_registration.rs
│   │   │   ├── user_invitation.rs
│   │   │   ├── user_preferences.rs
│   │   │   └── password_reset_token.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── user_registration_service.rs
│   │   │   ├── invitation_service.rs
│   │   │   ├── authentication_service.rs
│   │   │   ├── user_management_service.rs
│   │   │   ├── verification_service.rs
│   │   │   ├── password_reset_service.rs
│   │   │   ├── preference_service.rs
│   │   │   └── rate_limit_service.rs
│   │   └── value_objects/
│   │       ├── mod.rs
│   │       ├── email.rs
│   │       ├── username.rs
│   │       └── token.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── user_services.rs
│   │   ├── registration_services.rs
│   │   ├── invitation_services.rs
│   │   ├── auth_services.rs
│   │   ├── role_services.rs
│   │   └── preference_services.rs
│   ├── domain_events/
│   │   ├── mod.rs
│   │   └── events.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── keycloak_client.rs
│   │   ├── repositories.rs
│   │   └── postgres.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   │   └── routes.rs
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── user_dto.rs
│   │   ├── registration_dto.rs
│   │   ├── invitation_dto.rs
│   │   └── auth_dto.rs
│   ├── errors/
│   │   ├── mod.rs
│   │   └── app_error.rs
│   └── utils/
│       ├── mod.rs
│       ├── hash.rs
│       └── jwt.rs
├── migrations/
│   ├── 20240101000001_create_users_table.sql
│   ├── 20240101000002_create_registrations_table.sql
│   ├── 20240101000003_create_login_audit_log.sql
│   └── ...
└── tests/
    ├── integration_tests.rs
    └── domain_tests.rs
