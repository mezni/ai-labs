-- ==========================================
-- Extensions
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS hstore;

-- ==========================================
-- Connector Types
CREATE TABLE connector_types (
    connector_type_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    standard VARCHAR(100),
    current_type TEXT NOT NULL CHECK (current_type IN ('AC', 'DC')),
    typical_power_kw DECIMAL(6, 2),
    pin_configuration VARCHAR(100),
    is_public_standard BOOLEAN DEFAULT TRUE,
    created_by TEXT NOT NULL,
    updated_by TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- Networks
CREATE TABLE networks (
    network_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    type VARCHAR(20) NOT NULL CHECK (type IN ('individual', 'company')),
    contact_email VARCHAR(255),
    phone_number VARCHAR(50),
    address TEXT,
    created_by TEXT NOT NULL,
    updated_by TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- Companies
CREATE TABLE companies (
    company_id SERIAL PRIMARY KEY,
    network_id INTEGER UNIQUE NOT NULL REFERENCES networks(network_id) ON DELETE CASCADE,
    business_registration_number VARCHAR(100),
    website_url VARCHAR(255),
    created_by TEXT NOT NULL,
    updated_by TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- Stations
CREATE TABLE stations (
    station_id SERIAL PRIMARY KEY,
    network_id INTEGER REFERENCES networks(network_id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100),
    postal_code VARCHAR(20),
    location GEOGRAPHY(Point, 4326) NOT NULL,
    tags HSTORE,
    osm_id BIGINT,
    status VARCHAR(20) NOT NULL DEFAULT 'onboarding' CHECK (status IN ('onboarding', 'verified', 'declined')),
    is_operational BOOLEAN DEFAULT TRUE,
    created_by TEXT NOT NULL,
    updated_by TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- Connectors
CREATE TABLE connectors (
    connector_id SERIAL PRIMARY KEY,
    station_id INTEGER NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    connector_type_id INTEGER NOT NULL REFERENCES connector_types(connector_type_id),
    power_level_kw DECIMAL(6,2) NOT NULL,
    status TEXT NOT NULL DEFAULT 'available' CHECK (status IN ('available','occupied','out_of_service','reserved','unavailable')),
    max_voltage INTEGER,
    max_amperage INTEGER,
    serial_number VARCHAR(100),
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    installation_date DATE,
    last_maintenance_date DATE,
    created_by TEXT NOT NULL,
    updated_by TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- Station Verification History
CREATE TABLE station_verification_history (
    station_verification_history_id SERIAL PRIMARY KEY,
    station_id INTEGER NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('onboarding','verified','declined')),
    updated_by TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notes TEXT
);

-- ==========================================
-- Indexes
CREATE INDEX idx_connector_types_current_type ON connector_types(current_type);
CREATE INDEX idx_networks_name ON networks(name);
CREATE INDEX idx_stations_network_id ON stations(network_id);
CREATE INDEX idx_stations_city ON stations(city);
CREATE INDEX idx_stations_state ON stations(state);
CREATE INDEX idx_stations_country ON stations(country);
CREATE INDEX idx_stations_location ON stations USING GIST(location);
CREATE INDEX idx_stations_tags ON stations USING GIN(tags);
CREATE INDEX idx_stations_osm_id ON stations(osm_id);
CREATE INDEX idx_connectors_station_id ON connectors(station_id);
CREATE INDEX idx_connectors_connector_type_id ON connectors(connector_type_id);
CREATE INDEX idx_connectors_status ON connectors(status);
CREATE INDEX idx_connectors_manufacturer ON connectors(manufacturer);
CREATE INDEX idx_connectors_model ON connectors(model);
CREATE INDEX idx_connectors_power_level_kw ON connectors(power_level_kw);
CREATE INDEX idx_station_verification_history_station_id ON station_verification_history(station_id);

-- ==========================================
-- set_updated_at function and triggers
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for auto-updating 'updated_at'
DO $$
DECLARE t RECORD;
BEGIN
    FOR t IN SELECT table_name FROM information_schema.tables
             WHERE table_schema='public' AND table_name IN ('connector_types','networks','companies','stations','connectors') LOOP
        EXECUTE format('CREATE TRIGGER trg_set_updated_at_%1$I BEFORE UPDATE ON %1$I FOR EACH ROW EXECUTE FUNCTION set_updated_at();', t.table_name);
    END LOOP;
END;
$$;