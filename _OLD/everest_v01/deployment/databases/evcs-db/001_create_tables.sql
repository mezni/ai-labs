-- ===================================
-- Enable extensions
-- ===================================
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS hstore;

------------------------------------------------------------
-- Lookup Tables (Enum-like)
------------------------------------------------------------

CREATE TABLE access_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE data_sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE connector_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE current_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE connector_statuses (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL UNIQUE,
    description TEXT
);

------------------------------------------------------------
-- Networks
------------------------------------------------------------

CREATE TABLE networks (
    network_id VARCHAR(32) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    network_type VARCHAR(20) NOT NULL
        CHECK (network_type IN ('INDIVIDUAL', 'COMPANY')),
    support_phone VARCHAR(50),
    support_email VARCHAR(255),
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(36),
    updated_by VARCHAR(36)
);

CREATE INDEX idx_networks_name ON networks (name);
CREATE INDEX idx_networks_type ON networks (network_type);

------------------------------------------------------------
-- Stations
------------------------------------------------------------

CREATE TABLE stations (
    station_id VARCHAR(32) PRIMARY KEY,
    osm_id BIGINT NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    address TEXT,
    location GEOGRAPHY(Point, 4326) NOT NULL,
    tags HSTORE,
    network_id VARCHAR(32)
        REFERENCES networks(network_id) ON DELETE SET NULL,
    created_by VARCHAR(36),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(36),
    updated_at TIMESTAMPTZ
);

CREATE INDEX idx_stations_location ON stations USING GIST (location);
CREATE INDEX idx_stations_osm_id ON stations (osm_id);
CREATE INDEX idx_stations_name ON stations (name);
CREATE INDEX idx_stations_network_id ON stations (network_id);

------------------------------------------------------------
-- Users
------------------------------------------------------------

CREATE TABLE users (
    user_id VARCHAR(36) PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL
        CHECK (role IN ('admin', 'partner', 'operator', 'user')),
    network_id VARCHAR(32)
        REFERENCES networks(network_id) ON DELETE SET NULL,
    station_id VARCHAR(32)
        REFERENCES stations(station_id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    created_by VARCHAR(36),
    updated_by VARCHAR(36)
);

CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_role ON users (role);
CREATE INDEX idx_users_network_id ON users (network_id);
CREATE INDEX idx_users_station_id ON users (station_id);

------------------------------------------------------------
-- Connectors
------------------------------------------------------------

CREATE TABLE connectors (
    connector_id VARCHAR(32) PRIMARY KEY,
    station_id VARCHAR(32) NOT NULL
        REFERENCES stations(station_id) ON DELETE CASCADE,
    connector_type_id INTEGER NOT NULL
        REFERENCES connector_types(id),
    status_id INTEGER NOT NULL
        REFERENCES connector_statuses(id),
    current_type_id INTEGER NOT NULL
        REFERENCES current_types(id),
    power_kw NUMERIC(5,2),
    voltage INTEGER,
    amperage INTEGER,
    count_available INTEGER NOT NULL DEFAULT 1
        CHECK (count_available >= 0),
    count_total INTEGER NOT NULL DEFAULT 1
        CHECK (count_total >= 1 AND count_total >= count_available),
    created_by VARCHAR(36),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(36),
    updated_at TIMESTAMPTZ,
    CONSTRAINT uq_connector UNIQUE (station_id, connector_type_id, current_type_id)
);

CREATE INDEX idx_connectors_station_id ON connectors (station_id);
CREATE INDEX idx_connectors_connector_type_id ON connectors (connector_type_id);
CREATE INDEX idx_connectors_status_id ON connectors (status_id);
CREATE INDEX idx_connectors_current_type_id ON connectors (current_type_id);

------------------------------------------------------------
-- User Reviews
------------------------------------------------------------

CREATE TABLE user_reviews (
    review_id VARCHAR(32) PRIMARY KEY,
    user_id VARCHAR(36),
    station_id VARCHAR(32) NOT NULL
        REFERENCES stations(station_id) ON DELETE CASCADE,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    review_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(36),
    updated_by VARCHAR(36)
);

CREATE INDEX idx_user_reviews_station_id ON user_reviews (station_id);
CREATE INDEX idx_user_reviews_user_id ON user_reviews (user_id);
CREATE INDEX idx_user_reviews_rating ON user_reviews (rating);
