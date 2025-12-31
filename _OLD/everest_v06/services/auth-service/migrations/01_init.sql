-- =============================================================================
-- Optional: Case-insensitive text (email / username)
-- =============================================================================
-- CREATE EXTENSION IF NOT EXISTS citext;

-- =============================================================================
-- 1. Helper Function: Auto-update updated_at
-- =============================================================================
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- 2. Users Table
-- =============================================================================
CREATE TABLE IF NOT EXISTS users (
    user_id         VARCHAR(32) PRIMARY KEY,
    keycloak_id     VARCHAR(255) UNIQUE NOT NULL,

    email           VARCHAR(255) UNIQUE NOT NULL,
    username        VARCHAR(100) UNIQUE NOT NULL,

    first_name      VARCHAR(100),
    last_name       VARCHAR(100),
    phone           VARCHAR(20),
    photo           TEXT,

    is_verified     BOOLEAN DEFAULT TRUE,
    role            VARCHAR(50) NOT NULL DEFAULT 'user',

    network_id      VARCHAR(32) NOT NULL DEFAULT '',
    station_id     VARCHAR(32) NOT NULL DEFAULT '',
    source          VARCHAR(20) NOT NULL DEFAULT 'web',

    is_active       BOOLEAN DEFAULT TRUE,
    deleted_at      TIMESTAMPTZ,
    last_login_at   TIMESTAMPTZ,

    created_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    created_by      VARCHAR(32),
    updated_by      VARCHAR(32),

    CONSTRAINT valid_role CHECK (
        role IN ('user', 'admin', 'partner', 'operator')
    ),
    CONSTRAINT valid_source CHECK (
        source IN ('web', 'internal')
    ),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE)
        OR
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);

CREATE TRIGGER update_user_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

-- =============================================================================
-- 3. User Registrations
-- =============================================================================
CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id     VARCHAR(32) PRIMARY KEY,

    email               VARCHAR(255) NOT NULL,
    username            VARCHAR(100) NOT NULL,

    first_name          VARCHAR(100),
    last_name           VARCHAR(100),
    phone               VARCHAR(20),

    verification_token  VARCHAR(255) NOT NULL,
    status              VARCHAR(20) NOT NULL DEFAULT 'pending',

    keycloak_id         VARCHAR(255) UNIQUE NOT NULL,
    user_id             VARCHAR(32),

    resend_count        INTEGER DEFAULT 0,
    expires_at          TIMESTAMPTZ NOT NULL,
    verified_at         TIMESTAMPTZ,

    created_at          TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    ip_address          VARCHAR(50),
    user_agent          TEXT,
    source              VARCHAR(20) NOT NULL DEFAULT 'web',

    CONSTRAINT valid_registration_status CHECK (
        status IN ('pending', 'verified', 'expired', 'cancelled')
    ),
    CONSTRAINT fk_registration_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE SET NULL
);

-- =============================================================================
-- 4. Refresh Tokens
-- =============================================================================
CREATE TABLE IF NOT EXISTS refresh_tokens (
    token_id        VARCHAR(32) PRIMARY KEY,
    user_id         VARCHAR(32) NOT NULL,

    refresh_token   TEXT NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,

    created_at      TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    revoked_at      TIMESTAMPTZ,

    ip_address      VARCHAR(50),
    user_agent      TEXT,

    CONSTRAINT fk_refresh_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

-- =============================================================================
-- 5. Indexes
-- =============================================================================

-- Users
CREATE INDEX IF NOT EXISTS idx_user_email
    ON users(email);

CREATE INDEX IF NOT EXISTS idx_user_username
    ON users(username);

CREATE INDEX IF NOT EXISTS idx_user_keycloak_id
    ON users(keycloak_id);

CREATE INDEX IF NOT EXISTS idx_user_active
    ON users(is_active)
    WHERE is_active = TRUE;

-- Registrations
CREATE INDEX IF NOT EXISTS idx_registration_email
    ON user_registrations(email);

CREATE INDEX IF NOT EXISTS idx_registration_token
    ON user_registrations(verification_token);

CREATE INDEX IF NOT EXISTS idx_registration_status
    ON user_registrations(status);

CREATE INDEX IF NOT EXISTS idx_registration_expires
    ON user_registrations(expires_at);

-- Refresh Tokens
CREATE INDEX IF NOT EXISTS idx_refresh_token_user
    ON refresh_tokens(user_id);

CREATE INDEX IF NOT EXISTS idx_refresh_token_expires
    ON refresh_tokens(expires_at);

CREATE INDEX IF NOT EXISTS idx_refresh_token_active
    ON refresh_tokens(revoked_at)
    WHERE revoked_at IS NULL;

-- =============================================================================
-- 6. Audit & Logs
-- =============================================================================
CREATE TABLE IF NOT EXISTS login_audit_log (
    log_id          BIGSERIAL PRIMARY KEY,
    user_id         VARCHAR(32),
    keycloak_id     VARCHAR(255),

    action          VARCHAR(50) NOT NULL,
    action_details  TEXT,

    ip_address      VARCHAR(50),
    user_agent      TEXT,

    success         BOOLEAN DEFAULT TRUE,
    error_message   TEXT,

    created_at      TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS keycloak_sync_log (
    log_id          BIGSERIAL PRIMARY KEY,
    user_id         VARCHAR(32),
    keycloak_id     VARCHAR(255),

    action          VARCHAR(50) NOT NULL,
    status          VARCHAR(20) NOT NULL,

    details         TEXT,
    error_message   TEXT,

    created_at      TIMESTAMP DEFAULT NOW(),

    CONSTRAINT valid_sync_action CHECK (
        action IN ('create', 'update', 'delete', 'role_update', 'status_update')
    ),
    CONSTRAINT valid_sync_status CHECK (
        status IN ('success', 'failed', 'skipped')
    )
);

CREATE TABLE IF NOT EXISTS email_verification_logs (
    log_id              BIGSERIAL PRIMARY KEY,
    user_id             VARCHAR(32),

    email               VARCHAR(255) NOT NULL,
    verification_token  VARCHAR(100),

    verified            BOOLEAN DEFAULT FALSE,
    ip_address          VARCHAR(50),

    created_at          TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_email_verification_user
    ON email_verification_logs(user_id);

CREATE INDEX IF NOT EXISTS idx_email_verification_email
    ON email_verification_logs(email);

CREATE INDEX IF NOT EXISTS idx_email_verification_created
    ON email_verification_logs(created_at);

-- =============================================================================
-- 7. Rate Limiting
-- =============================================================================
CREATE TABLE IF NOT EXISTS rate_limits (
    id              BIGSERIAL PRIMARY KEY,
    identifier      VARCHAR(255) NOT NULL,
    action          VARCHAR(50) NOT NULL,

    count           INTEGER DEFAULT 1,

    window_start    TIMESTAMP NOT NULL,
    window_end      TIMESTAMP NOT NULL,

    created_at      TIMESTAMP DEFAULT NOW(),

    UNIQUE (identifier, action, window_start)
);

CREATE INDEX IF NOT EXISTS idx_rate_limits_identifier
    ON rate_limits(identifier);

CREATE INDEX IF NOT EXISTS idx_rate_limits_action
    ON rate_limits(action);

CREATE INDEX IF NOT EXISTS idx_rate_limits_window_end
    ON rate_limits(window_end);

-- =============================================================================
-- 8. User Preferences
-- =============================================================================
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id                 VARCHAR(32) PRIMARY KEY,

    language                VARCHAR(10) DEFAULT 'en',
    timezone                VARCHAR(50) DEFAULT 'UTC',

    notifications_enabled   BOOLEAN DEFAULT TRUE,
    theme                   VARCHAR(20) DEFAULT 'light',

    preferences             JSONB,

    created_at              TIMESTAMP DEFAULT NOW(),
    updated_at              TIMESTAMP DEFAULT NOW(),

    FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);

-- =============================================================================
-- 9. User Invitations
-- =============================================================================
CREATE TABLE IF NOT EXISTS user_invitations (
    invitation_id   VARCHAR(32) PRIMARY KEY,

    email           VARCHAR(255) NOT NULL,
    role            VARCHAR(50) NOT NULL,

    network_id      VARCHAR(32),
    station_id      VARCHAR(32),

    invited_by      VARCHAR(32) NOT NULL,

    token           VARCHAR(100) UNIQUE NOT NULL,
    status          VARCHAR(20) NOT NULL DEFAULT 'pending',

    expires_at      TIMESTAMP NOT NULL,
    accepted_at     TIMESTAMP,

    created_at      TIMESTAMP DEFAULT NOW(),

    CONSTRAINT valid_invitation_role CHECK (
        role IN ('admin', 'partner', 'operator')
    ),
    CONSTRAINT valid_invitation_status CHECK (
        status IN ('pending', 'accepted', 'expired', 'cancelled')
    )
);

-- =============================================================================
-- Table Comments
-- =============================================================================
COMMENT ON TABLE users IS 'Verified user accounts synchronized with Keycloak';
COMMENT ON TABLE user_registrations IS 'Temporary storage for pending user registrations';
COMMENT ON TABLE refresh_tokens IS 'OIDC refresh token lifecycle management';
