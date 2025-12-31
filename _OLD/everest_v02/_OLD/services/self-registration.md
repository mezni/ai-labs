sequenceDiagram
    autonumber
    actor User
    participant API as Auth Service
    participant DB as PostgreSQL
    participant KC as Keycloak

    User->>API: Submit registration data
    API->>DB: INSERT user_registrations (status=pending)
    API->>KC: Create user (enabled=false, send verification)
    KC-->>API: keycloak_id
    API->>DB: UPDATE user_registrations (keycloak_id)
    API->>DB: INSERT keycloak_sync_log (create, success)

    User->>KC: Click verification link
    KC->>API: Verification callback (keycloak_id)
    API->>DB: UPDATE user_registrations (status=verified, verified_at)
    API->>DB: INSERT users (is_verified=true, source=web)
    API->>DB: UPDATE user_registrations (user_id)
    API->>DB: INSERT user_preferences
    API->>KC: Enable user
    API->>DB: INSERT keycloak_sync_log (status_update, success)


OpenAPI 3.1 – Auth Service (Self-Registration Only)
openapi: 3.1.0
info:
  title: Auth Service API
  description: >
    Authentication service supporting user self-registration with
    Keycloak-managed email verification.
  version: 1.0.0

servers:
  - url: https://api.example.com
    description: Production
  - url: http://localhost:8080
    description: Local development

tags:
  - name: Registration
    description: User self-registration flow
  - name: Verification
    description: Email verification callbacks

paths:

  /auth/register:
    post:
      tags: [Registration]
      summary: Register a new user (self-registration)
      description: >
        Creates a pending user registration and triggers Keycloak
        to send a verification email.
      operationId: registerUser
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RegisterUserRequest'
      responses:
        '201':
          description: Registration created successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/RegisterUserResponse'
        '400':
          description: Invalid input
        '409':
          description: Email or username already registered
        '429':
          description: Too many registration attempts
        '500':
          description: Internal server error

  /auth/verify/callback:
    post:
      tags: [Verification]
      summary: Keycloak email verification callback
      description: >
        Callback endpoint invoked by Keycloak after a user verifies
        their email. This endpoint finalizes user creation.
      operationId: verifyUserCallback
      security:
        - keycloakWebhook: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/KeycloakVerificationCallback'
      responses:
        '204':
          description: Verification processed successfully
        '400':
          description: Invalid callback payload
        '404':
          description: Registration not found
        '409':
          description: User already verified
        '500':
          description: Internal server error

components:

  securitySchemes:
    keycloakWebhook:
      type: apiKey
      in: header
      name: X-Keycloak-Signature
      description: >
        Signature header to validate Keycloak webhook authenticity.

  schemas:

    RegisterUserRequest:
      type: object
      required:
        - email
        - username
      properties:
        email:
          type: string
          format: email
          example: user@example.com
        username:
          type: string
          minLength: 3
          maxLength: 100
          example: johndoe
        first_name:
          type: string
          maxLength: 100
          example: John
        last_name:
          type: string
          maxLength: 100
          example: Doe
        phone:
          type: string
          maxLength: 20
          example: "+15551234567"

    RegisterUserResponse:
      type: object
      properties:
        registration_id:
          type: string
          example: "b3d9f3d9eacb4d3f9c9a2d1b3f0a9e8a"
        status:
          type: string
          enum: [pending]
          example: pending
        expires_at:
          type: string
          format: date-time
          example: "2025-01-02T12:00:00Z"

    KeycloakVerificationCallback:
      type: object
      required:
        - keycloak_id
        - email
        - verified
      properties:
        keycloak_id:
          type: string
          example: "9b7e4f9c-1c9d-4b1f-9c44-abc123"
        email:
          type: string
          format: email
          example: user@example.com
        verified:
          type: boolean
          example: true
        verified_at:
          type: string
          format: date-time
          example: "2025-01-01T10:15:00Z"




auth-service/
├── Cargo.toml
├── README.md
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│
│   ├── core/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── logging.rs
│   │   ├── database.rs
│   │   ├── errors.rs
│   │   ├── constants.rs
│   │   ├── jwt.rs
│   │   ├── middleware.rs
│   │   └── utils.rs
│
│   ├── interfaces/
│   │   ├── mod.rs
│   │   ├── openapi.rs
│   │   │
│   │   ├── http/
│   │   │   ├── mod.rs
│   │   │   ├── routes.rs
│   │   │   ├── register_user.rs
│   │   │   ├── verification_callback.rs
│   │   │   └── keycloak_middleware.rs
│   │   │
│   │   └── grpc/
│   │       └── mod.rs
│
│   ├── application/
│   │   ├── mod.rs
│   │   ├── user_registration_service.rs
│   │   ├── verification_callback_service.rs
│   │   └── registration_cleanup_service.rs
│
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── user_registration.rs
│   │   ├── value_objects.rs
│   │   ├── repositories.rs
│   │   ├── events.rs
│   │   └── errors.rs
│
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── db.rs
│   │   ├── repositories_pg.rs
│   │   ├── keycloak_client.rs
│   │   ├── cache.rs
│   │   └── queue.rs
│
│   ├── jobs/
│   │   ├── mod.rs
│   │   └── registration_cleanup_job.rs
│
│   └── tests/
│       ├── integration/
│       │   └── user_registration_test.rs
│       └── unit/
│           └── user_test.rs
│
├── migrations/
│   └── *.sql
│
├── openapi/
│   └── auth-service.yaml
│
├── scripts/
│   ├── build.sh
│   └── deploy.sh
│
├── Dockerfile
├── docker-compose.yml
└── Makefile



Endpoint Summary Table
Method	Path	Auth	Purpose
POST	/api/v1/register	❌	Start self-registration
POST	/api/v1/register/verify	🔐 KC	Email verification callback
GET	/api/v1/register/status	❌	Check registration status
POST	/internal/v1/register/cleanup	🔐	Expire registrations
GET	/api/v1/health	❌	Liveness
GET	/api/v1/ready	❌	Readiness


stack
actix-cors = "0.7.1"
actix-web = "4.12.1"

# Database
sqlx = { version = "0.8.6", features = ["runtime-tokio-native-tls", "postgres", "chrono", "uuid"] }

# Async Runtime
tokio = { version = "1.48.0", features = ["full"] }

# Serialization
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"

# Error Handling
anyhow = "1.0.100"
thiserror = "2.0.17"


# Logging
tracing = "0.1.43"
tracing-subscriber = { version = "0.3.22", features = ["env-filter"] }

# Configuration
dotenvy = "0.15.7"


# JWT & Crypto
jsonwebtoken = { version = "10.2.0", features = ["rust_crypto"] }
reqwest = { version = "0.12.25", features = ["json"] }

# Time
chrono = { version = "0.4.42", features = ["serde"] }

# ID Generation
nanoid = "0.4.0"

# OpenAPI
utoipa = { version = "5.4.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = { version = "9.0.2", features = ["actix-web"] }

# Async utilities
futures = "0.3.31"
async-trait = "0.1.89


# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info,auth_service=debug

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5800/auth_db
DATABASE_MAX_CONNECTIONS=10


# Keycloak Configuration (Auth Service specific)
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_REALM=myrealm
KEYCLOAK_AUTH_CLIENT_ID=auth-client
KEYCLOAK_BACKEND_CLIENT_ID=backend-admin
KEYCLOAK_BACKEND_CLIENT_SECRET=your-backend-admin-secret-here

registration_id is REG-nanoid(18)
user_id is USR-nanoid(18)

CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    verification_token VARCHAR(100) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    keycloak_id VARCHAR(255),
    user_id VARCHAR(32),
    expires_at TIMESTAMP NOT NULL,
    verified_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT chk_registration_status CHECK (status IN ('pending', 'verified', 'expired', 'cancelled'))
);

CREATE UNIQUE INDEX ux_registration_pending_email
    ON user_registrations (LOWER(email))
    WHERE status = 'pending';

CREATE INDEX idx_registration_status ON user_registrations(status);
CREATE INDEX idx_registration_expires_at ON user_registrations(expires_at);
CREATE INDEX idx_registration_keycloak_id ON user_registrations(keycloak_id);


CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT TRUE,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT chk_user_role CHECK (role IN ('user','admin','partner','operator')),
    CONSTRAINT chk_user_source CHECK (source IN ('web','internal'))
);

CREATE UNIQUE INDEX ux_users_email ON users(LOWER(email));
CREATE UNIQUE INDEX ux_users_username ON users(LOWER(username));
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);

CREATE TABLE IF NOT EXISTS user_preferences (
    user_id VARCHAR(32) PRIMARY KEY,
    language VARCHAR(10) DEFAULT 'en',
    timezone VARCHAR(50) DEFAULT 'UTC',
    notifications_enabled BOOLEAN DEFAULT TRUE,
    theme VARCHAR(20) DEFAULT 'light',
    preferences JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS keycloak_sync_log (
    log_id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(32),
    keycloak_id VARCHAR(255),
    action VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    details TEXT,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT chk_sync_action CHECK (action IN ('create','enable','disable','delete','role_update')),
    CONSTRAINT chk_sync_status CHECK (status IN ('success','failed','skipped'))
);

CREATE INDEX idx_sync_user_id ON keycloak_sync_log(user_id);
CREATE INDEX idx_sync_keycloak_id ON keycloak_sync_log(keycloak_id);
CREATE INDEX idx_sync_action ON keycloak_sync_log(action);
CREATE INDEX idx_sync_status ON keycloak_sync_log(status);
CREATE INDEX idx_sync_created_at ON keycloak_sync_log(created_at);


CREATE TABLE IF NOT EXISTS rate_limits (
    id BIGSERIAL PRIMARY KEY,
    identifier VARCHAR(255) NOT NULL,
    action VARCHAR(50) NOT NULL,
    count INTEGER DEFAULT 1,
    window_start TIMESTAMP NOT NULL,
    window_end TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(identifier, action, window_start)
);

CREATE INDEX idx_rate_limits_identifier ON rate_limits(identifier);
CREATE INDEX idx_rate_limits_action ON rate_limits(action);
CREATE INDEX idx_rate_limits_window_end ON rate_limits(window_end);
