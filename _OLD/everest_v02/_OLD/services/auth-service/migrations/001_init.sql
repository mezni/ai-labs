-- User Registrations Table
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

-- Users Table
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

-- User Preferences Table
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

-- Keycloak Sync Log Table
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

-- Rate Limits Table
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