CREATE TABLE user_registrations (
    -- Primary Identifier
    registration_id VARCHAR(32) PRIMARY KEY,

    -- User Details
    email VARCHAR(255) NOT NULL,
    email_normalized VARCHAR(255)
        GENERATED ALWAYS AS (LOWER(email)) STORED,
    username VARCHAR(100),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),

    -- Verification (SECURE: hash only)
    verification_token_hash TEXT NOT NULL,
    verification_code VARCHAR(10),
    verification_token_expires_at TIMESTAMPTZ NOT NULL,
    verification_method VARCHAR(50) NOT NULL DEFAULT 'email',
    verified_at TIMESTAMPTZ,
    verification_ip INET,
    verification_user_agent TEXT,

    -- Keycloak Integration
    keycloak_id VARCHAR(255),
    keycloak_sync_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    keycloak_sync_attempts INTEGER NOT NULL DEFAULT 0,
    keycloak_last_sync_at TIMESTAMPTZ,
    keycloak_sync_error TEXT,
    keycloak_sync_metadata JSONB NOT NULL DEFAULT '{}'::JSONB,

    -- Source & Context
    source VARCHAR(50) NOT NULL DEFAULT 'web',
    registration_ip INET,
    user_agent TEXT,
    referrer TEXT,
    campaign_id VARCHAR(100),
    utm_source VARCHAR(100),
    utm_medium VARCHAR(100),
    utm_campaign VARCHAR(100),

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    status_reason TEXT,

    -- Lifecycle Expiry
    expires_at TIMESTAMPTZ NOT NULL,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT user_registrations_status_check
        CHECK (status IN (
            'pending',
            'verified',
            'expired',
            'cancelled',
            'failed',
            'keycloak_failed',
            'email_failed'
        )),

    CONSTRAINT user_registrations_source_check
        CHECK (source IN (
            'web',
            'mobile',
            'api',
            'admin',
            'invitation'
        )),

    CONSTRAINT user_registrations_verification_method_check
        CHECK (verification_method IN (
            'email',
            'sms',
            'manual'
        )),

    CONSTRAINT user_registrations_sync_status_check
        CHECK (keycloak_sync_status IN (
            'pending',
            'in_progress',
            'success',
            'failed',
            'retry'
        ))
);


CREATE UNIQUE INDEX ux_user_registrations_pending_email
    ON user_registrations (email_normalized)
    WHERE status = 'pending';

CREATE UNIQUE INDEX ux_user_registrations_keycloak_id
    ON user_registrations (keycloak_id)
    WHERE keycloak_id IS NOT NULL;

CREATE INDEX ix_user_registrations_token_hash
    ON user_registrations (verification_token_hash);


CREATE TABLE users (
    -- Primary Identifiers
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) NOT NULL UNIQUE,
    external_id VARCHAR(255),

    -- Traceability
    registration_id VARCHAR(32)
        REFERENCES user_registrations(registration_id),

    -- Personal Information
    email VARCHAR(255) NOT NULL UNIQUE,
    email_normalized VARCHAR(255)
        GENERATED ALWAYS AS (LOWER(email)) STORED,

    username VARCHAR(100) UNIQUE,
    username_normalized VARCHAR(100)
        GENERATED ALWAYS AS (LOWER(username)) STORED,

    first_name VARCHAR(100),
    last_name VARCHAR(100),

    display_name VARCHAR(200)
        GENERATED ALWAYS AS (
            COALESCE(
                NULLIF(first_name || ' ' || last_name, ' '),
                username,
                email
            )
        ) STORED,

    phone VARCHAR(20),
    phone_normalized VARCHAR(20),
    avatar_url TEXT,
    date_of_birth DATE,
    gender VARCHAR(20),

    -- Authorization
    role VARCHAR(50) NOT NULL DEFAULT 'user',

    -- Status Flags
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    status_reason TEXT,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    phone_verified BOOLEAN NOT NULL DEFAULT FALSE,
    requires_password_change BOOLEAN NOT NULL DEFAULT FALSE,

    -- MFA
    mfa_method VARCHAR(20),

    -- Security & Lockout
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    failed_password_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TIMESTAMPTZ,
    last_failed_login TIMESTAMPTZ,

    -- Source & Metadata
    source VARCHAR(50) NOT NULL DEFAULT 'web',
    source_details JSONB NOT NULL DEFAULT '{}'::JSONB,
    registration_ip INET,
    registration_user_agent TEXT,

    -- Activity Tracking
    last_login_at TIMESTAMPTZ,
    last_login_ip INET,
    last_login_user_agent TEXT,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    login_count INTEGER NOT NULL DEFAULT 0,

    -- Privacy & Compliance
    accepted_terms_at TIMESTAMPTZ,
    accepted_privacy_at TIMESTAMPTZ,
    marketing_consent BOOLEAN NOT NULL DEFAULT FALSE,
    data_processing_consent BOOLEAN NOT NULL DEFAULT FALSE,
    gdpr_anonymized_at TIMESTAMPTZ,

    -- Audit & Lifecycle
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    version INTEGER NOT NULL DEFAULT 1,

    -- Constraints
    CONSTRAINT users_status_check
        CHECK (status IN (
            'pending',
            'active',
            'inactive',
            'locked',
            'suspended',
            'deleted'
        )),

    CONSTRAINT users_source_check
        CHECK (source IN (
            'web',
            'mobile',
            'api',
            'admin',
            'import',
            'sso'
        )),

    CONSTRAINT users_mfa_method_check
        CHECK (
            mfa_method IS NULL OR
            mfa_method IN ('totp', 'sms', 'email', 'webauthn')
        ),

    CONSTRAINT users_gender_check
        CHECK (
            gender IS NULL OR
            gender IN (
                'male',
                'female',
                'other'
            )
        ),

    CONSTRAINT users_role_check
        CHECK (role IN (
            'user',
            'admin',
            'partner',
            'operator'
        )),

    CONSTRAINT users_deleted_check
        CHECK (
            deleted_at IS NULL OR status = 'deleted'
        )
);

CREATE INDEX ix_users_email_normalized
    ON users (email_normalized);

CREATE INDEX ix_users_username_normalized
    ON users (username_normalized);

CREATE INDEX ix_users_last_login
    ON users (last_login_at);

CREATE INDEX ix_users_status
    ON users (status);
