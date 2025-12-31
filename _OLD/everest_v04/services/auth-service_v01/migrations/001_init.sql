-- Create users table
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
    deleted_at TIMESTAMP,
    last_login_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
    
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
    CONSTRAINT valid_source CHECK (source IN ('web', 'mobile', 'internal')),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE) OR 
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);

-- Create user_registrations table
CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    verification_token VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    user_id VARCHAR(32),
    resend_count INTEGER DEFAULT 0,
    expires_at TIMESTAMP NOT NULL,
    verified_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    ip_address VARCHAR(50),
    user_agent TEXT,
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    
    CONSTRAINT valid_registration_status CHECK (status IN ('pending', 'verified', 'expired', 'cancelled')),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE SET NULL
);

-- Create refresh_tokens table
CREATE TABLE IF NOT EXISTS refresh_tokens (
    token_id VARCHAR(32) PRIMARY KEY,
    user_id VARCHAR(32) NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    revoked_at TIMESTAMP,
    ip_address VARCHAR(50),
    user_agent TEXT,
    
    CONSTRAINT fk_refresh_user FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_user_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_user_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_user_keycloak_id ON users(keycloak_id);
CREATE INDEX IF NOT EXISTS idx_user_is_active ON users(is_active) WHERE is_active = TRUE;

CREATE INDEX IF NOT EXISTS idx_registration_email ON user_registrations(email);
CREATE INDEX IF NOT EXISTS idx_registration_token ON user_registrations(verification_token);
CREATE INDEX IF NOT EXISTS idx_registration_status ON user_registrations(status);
CREATE INDEX IF NOT EXISTS idx_registration_expires ON user_registrations(expires_at);

CREATE INDEX IF NOT EXISTS idx_refresh_token_user ON refresh_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_refresh_token_expires ON refresh_tokens(expires_at);
CREATE INDEX IF NOT EXISTS idx_refresh_token_revoked ON refresh_tokens(revoked_at) WHERE revoked_at IS NULL;

-- Comments for documentation
COMMENT ON TABLE users IS 'Stores verified user accounts';
COMMENT ON TABLE user_registrations IS 'Temporary storage for pending user registrations';
COMMENT ON TABLE refresh_tokens IS 'Stores refresh tokens for session management';

COMMENT ON COLUMN users.keycloak_id IS 'Unique identifier from Keycloak';
COMMENT ON COLUMN users.is_verified IS 'Always true for users created after verification';
COMMENT ON COLUMN user_registrations.verification_token IS 'Unique token sent to user email for verification';
COMMENT ON COLUMN user_registrations.expires_at IS 'Timestamp when the registration expires (default 24 hours)';
COMMENT ON COLUMN refresh_tokens.revoked_at IS 'Timestamp when token was revoked, NULL if still valid';