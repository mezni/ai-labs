-- Enable extension if you want case-insensitive email/usernames (optional)
-- CREATE EXTENSION IF NOT EXISTS citext;

-- =============================================================================
-- 1. Helper Function for Automatic updated_at
-- =============================================================================
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- =============================================================================
-- 2. Users Table
-- =============================================================================
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN DEFAULT TRUE,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    network_id VARCHAR(32) NOT NULL DEFAULT '',
    station_id VARCHAR(32) NOT NULL DEFAULT '',
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    is_active BOOLEAN DEFAULT TRUE,
    deleted_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
    
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
    CONSTRAINT valid_source CHECK (source IN ('web', 'internal')),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE) OR 
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);

CREATE TRIGGER update_user_modtime
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE PROCEDURE update_modified_column();

-- =============================================================================
-- 3. User Registrations Table
-- =============================================================================
CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    verification_token VARCHAR(255)  NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    user_id VARCHAR(32),
    resend_count INTEGER DEFAULT 0,
    expires_at TIMESTAMPTZ NOT NULL,
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    ip_address VARCHAR(50),
    user_agent TEXT,
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    
    CONSTRAINT valid_registration_status CHECK (status IN ('pending', 'verified', 'expired', 'cancelled')),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE SET NULL
);

-- =============================================================================
-- 4. Refresh Tokens Table
-- =============================================================================
CREATE TABLE IF NOT EXISTS refresh_tokens (
    token_id VARCHAR(32) PRIMARY KEY,
    user_id VARCHAR(32) NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    revoked_at TIMESTAMPTZ,
    ip_address VARCHAR(50),
    user_agent TEXT,
    
    CONSTRAINT fk_refresh_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- =============================================================================
-- 5. Indexes
-- =============================================================================
-- User Indexes
CREATE INDEX IF NOT EXISTS idx_user_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_user_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_user_keycloak_id ON users(keycloak_id);
-- Partial index for active users (smaller and faster)
CREATE INDEX IF NOT EXISTS idx_user_is_active ON users(is_active) WHERE is_active = TRUE;

-- Registration Indexes
CREATE INDEX IF NOT EXISTS idx_registration_email ON user_registrations(email);
CREATE INDEX IF NOT EXISTS idx_registration_token ON user_registrations(verification_token);
CREATE INDEX IF NOT EXISTS idx_registration_status ON user_registrations(status);
CREATE INDEX IF NOT EXISTS idx_registration_expires ON user_registrations(expires_at);

-- Token Indexes
CREATE INDEX IF NOT EXISTS idx_refresh_token_user ON refresh_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_token_expires ON refresh_tokens(expires_at);
-- Partial index for valid tokens only
CREATE INDEX IF NOT EXISTS idx_refresh_token_revoked ON refresh_tokens(revoked_at) WHERE revoked_at IS NULL;

-- =============================================================================
-- 6. Comments
-- =============================================================================
COMMENT ON TABLE users IS 'Verified user accounts sync with Keycloak';
COMMENT ON TABLE user_registrations IS 'Temporary storage for pending sign-ups';
COMMENT ON TABLE refresh_tokens IS 'OIDC session management tokens';