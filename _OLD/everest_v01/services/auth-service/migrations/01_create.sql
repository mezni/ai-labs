-- migrations/20240101000001_create_users_table.sql
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    network_id VARCHAR(32) NOT NULL DEFAULT '',
    station_id VARCHAR(32) NOT NULL DEFAULT '',
    source VARCHAR(20) NOT NULL DEFAULT 'web',
    is_active BOOLEAN DEFAULT TRUE,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
    
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
    CONSTRAINT valid_source CHECK (source IN ('web', 'internal')),
    CONSTRAINT check_deleted CHECK (
        (deleted_at IS NULL AND is_active = TRUE) OR 
        (deleted_at IS NOT NULL AND is_active = FALSE)
    )
);

CREATE INDEX idx_users_email ON users(LOWER(email));
CREATE INDEX idx_users_username ON users(LOWER(username));
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_network_id ON users(network_id);
CREATE INDEX idx_users_station_id ON users(station_id);
CREATE INDEX idx_users_source ON users(source);
CREATE INDEX idx_users_is_active ON users(is_active);
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

-- migrations/20240101000002_create_registrations_table.sql
CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    verification_token VARCHAR(100) UNIQUE NOT NULL,
    verification_code VARCHAR(10),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    keycloak_id VARCHAR(255),
    user_id VARCHAR(32),
    expires_at TIMESTAMP NOT NULL,
    verified_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    ip_address VARCHAR(50),
    user_agent TEXT,
    
    CONSTRAINT valid_registration_status CHECK (status IN ('pending', 'verified', 'expired', 'cancelled'))
);

CREATE INDEX idx_registrations_email ON user_registrations(LOWER(email));
CREATE INDEX idx_registrations_token ON user_registrations(verification_token);
CREATE INDEX idx_registrations_status ON user_registrations(status);
CREATE INDEX idx_registrations_expires_at ON user_registrations(expires_at);

-- migrations/20240101000003_create_login_audit_log.sql
CREATE TABLE IF NOT EXISTS login_audit_log (
    log_id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(32),
    keycloak_id VARCHAR(255),
    action VARCHAR(50) NOT NULL,
    action_details TEXT,
    ip_address VARCHAR(50),
    user_agent TEXT,
    success BOOLEAN DEFAULT TRUE,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_login_audit_user_id ON login_audit_log(user_id);
CREATE INDEX idx_login_audit_action ON login_audit_log(action);
CREATE INDEX idx_login_audit_created_at ON login_audit_log(created_at);
CREATE INDEX idx_login_audit_success ON login_audit_log(success);

-- migrations/20240101000004_create_keycloak_sync_log.sql
CREATE TABLE IF NOT EXISTS keycloak_sync_log (
    log_id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(32),
    keycloak_id VARCHAR(255),
    action VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    details TEXT,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    
    CONSTRAINT valid_sync_action CHECK (action IN ('create', 'update', 'delete', 'role_update', 'status_update')),
    CONSTRAINT valid_sync_status CHECK (status IN ('success', 'failed', 'skipped'))
);

CREATE INDEX idx_sync_log_user_id ON keycloak_sync_log(user_id);
CREATE INDEX idx_sync_log_keycloak_id ON keycloak_sync_log(keycloak_id);
CREATE INDEX idx_sync_log_action ON keycloak_sync_log(action);
CREATE INDEX idx_sync_log_status ON keycloak_sync_log(status);
CREATE INDEX idx_sync_log_created_at ON keycloak_sync_log(created_at);

-- migrations/20240101000005_create_password_reset_tokens.sql
CREATE TABLE IF NOT EXISTS password_reset_tokens (
    token_id VARCHAR(32) PRIMARY KEY,
    user_id VARCHAR(32) NOT NULL,
    email VARCHAR(255) NOT NULL,
    token VARCHAR(100) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    ip_address VARCHAR(50),
    
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE INDEX idx_password_reset_token ON password_reset_tokens(token);
CREATE INDEX idx_password_reset_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_expires_at ON password_reset_tokens(expires_at);

-- migrations/20240101000006_create_email_verification_logs.sql
CREATE TABLE IF NOT EXISTS email_verification_logs (
    log_id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(32),
    email VARCHAR(255) NOT NULL,
    verification_token VARCHAR(100),
    verified BOOLEAN DEFAULT FALSE,
    ip_address VARCHAR(50),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_email_verification_user_id ON email_verification_logs(user_id);
CREATE INDEX idx_email_verification_email ON email_verification_logs(email);
CREATE INDEX idx_email_verification_created_at ON email_verification_logs(created_at);

-- migrations/20240101000007_create_rate_limits.sql
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

-- migrations/20240101000008_create_user_preferences.sql
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

-- migrations/20240101000009_create_user_invitations.sql
CREATE TABLE IF NOT EXISTS user_invitations (
    invitation_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    network_id VARCHAR(32),
    station_id VARCHAR(32),
    invited_by VARCHAR(32) NOT NULL,
    token VARCHAR(100) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    expires_at TIMESTAMP NOT NULL,
    accepted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    
    CONSTRAINT valid_invitation_role CHECK (role IN ('admin', 'partner', 'operator')),
    CONSTRAINT valid_invitation_status CHECK (status IN ('pending', 'accepted', 'expired', 'cancelled'))
);

CREATE INDEX idx_invitations_email ON user_invitations(email);
CREATE INDEX idx_invitations_token ON user_invitations(token);
CREATE INDEX idx_invitations_status ON user_invitations(status);