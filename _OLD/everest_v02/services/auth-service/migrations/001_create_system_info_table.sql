CREATE TABLE IF NOT EXISTS system_info (
    id SERIAL PRIMARY KEY,
    key VARCHAR(255) UNIQUE NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Seed initial data
INSERT INTO system_info (key, value, description) 
VALUES ('maintenance_mode', 'false', 'Flag to disable API access');

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

CREATE TABLE IF NOT EXISTS user_registrations (
    registration_id VARCHAR(32) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    verification_token VARCHAR(100),
    verification_code VARCHAR(10),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    keycloak_id VARCHAR(255),
    user_id VARCHAR(32),
    expires_at TIMESTAMP NOT NULL,
    verified_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    ip_address VARCHAR(50),
    user_agent TEXT,
    
    CONSTRAINT valid_registration_status CHECK (status IN ('pending', 'verified', 'expired', 'cancelled'))
);

CREATE INDEX idx_registrations_email ON user_registrations(LOWER(email));
CREATE INDEX idx_registrations_token ON user_registrations(verification_token);
CREATE INDEX idx_registrations_status ON user_registrations(status);
CREATE INDEX idx_registrations_expires_at ON user_registrations(expires_at);
