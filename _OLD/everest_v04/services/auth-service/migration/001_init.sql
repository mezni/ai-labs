-- =====================================================
-- USER REGISTRATIONS (PRE-ACCOUNT STATE)
-- =====================================================
CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,

    email VARCHAR(255) NOT NULL,
    email_normalized VARCHAR(255)
        GENERATED ALWAYS AS (LOWER(email)) STORED,

    username VARCHAR(100) NOT NULL,
    username_normalized VARCHAR(100)
        GENERATED ALWAYS AS (LOWER(username)) STORED,

    first_name VARCHAR(100),
    last_name VARCHAR(100),

    phone VARCHAR(20),
    phone_normalized VARCHAR(20)
        GENERATED ALWAYS AS (regexp_replace(phone, '\D', '', 'g')) STORED,

    verification_token VARCHAR(255),
    verification_code VARCHAR(10),
    verification_method VARCHAR(50) NOT NULL DEFAULT 'email',

    expires_at TIMESTAMPTZ NOT NULL,

    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    status_reason TEXT,

    keycloak_id VARCHAR(255) UNIQUE,
    user_id VARCHAR(32),

    resend_count INTEGER NOT NULL DEFAULT 0,
    verified_at TIMESTAMPTZ,

    source VARCHAR(20) NOT NULL DEFAULT 'web',

    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT user_registrations_status_check
        CHECK (status IN ('pending', 'verified', 'expired', 'cancelled')),

    CONSTRAINT user_registrations_source_check
        CHECK (source IN ('web', 'mobile', 'api', 'admin', 'invitation')),

    CONSTRAINT user_registrations_verification_method_check
        CHECK (verification_method IN ('email', 'sms'))
);

CREATE INDEX idx_user_registrations_email_norm
    ON user_registrations (email_normalized);

CREATE INDEX idx_user_registrations_username_norm
    ON user_registrations (username_normalized);

CREATE INDEX idx_user_registrations_status
    ON user_registrations (status);

-- =====================================================
-- USERS (ACTIVE ACCOUNT AGGREGATE)
-- =====================================================
CREATE TABLE IF NOT EXISTS users (
    -- Primary Identifiers
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) NOT NULL UNIQUE,
    external_id VARCHAR(255),

    -- Traceability
    registration_id VARCHAR(32)
        REFERENCES user_registrations(registration_id),

    -- Identity
    email VARCHAR(255) NOT NULL UNIQUE,
    email_normalized VARCHAR(255)
        GENERATED ALWAYS AS (LOWER(email)) STORED,

    username VARCHAR(100) UNIQUE,
    username_normalized VARCHAR(100)
        GENERATED ALWAYS AS (LOWER(username)) STORED,

    first_name VARCHAR(100),
    last_name VARCHAR(100),

    phone VARCHAR(20),
    phone_normalized VARCHAR(20)
        GENERATED ALWAYS AS (regexp_replace(phone, '\D', '', 'g')) STORED,

    avatar_url TEXT,
    date_of_birth DATE,
    gender VARCHAR(20),

    -- Authorization
    role VARCHAR(50) NOT NULL DEFAULT 'user',

    -- Account Status
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
            gender IN ('male', 'female', 'other')
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
            (deleted_at IS NULL AND status <> 'deleted')
         OR (deleted_at IS NOT NULL AND status = 'deleted')
        )
);

CREATE INDEX idx_users_email_norm
    ON users (email_normalized);

CREATE INDEX idx_users_username_norm
    ON users (username_normalized);

CREATE INDEX idx_users_status
    ON users (status);

CREATE INDEX idx_users_registration_id
    ON users (registration_id);

