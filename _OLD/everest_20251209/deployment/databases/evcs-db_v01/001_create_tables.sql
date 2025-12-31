-- Complete EV Charging Station Schema

-- Networks
CREATE TABLE IF NOT EXISTS networks (
    network_id        VARCHAR(32) PRIMARY KEY,
    name              VARCHAR(255) NOT NULL,
    network_type      VARCHAR(20) NOT NULL CHECK (network_type IN ('INDIVIDUAL', 'COMPANY')),
    support_phone     VARCHAR(50),
    support_email     VARCHAR(255),
    is_verified       BOOLEAN DEFAULT FALSE,
    is_active         BOOLEAN DEFAULT TRUE,
    is_live           BOOLEAN DEFAULT TRUE,
    created_by        VARCHAR(32) NOT NULL,
    updated_by        VARCHAR(32),
    created_at        TIMESTAMPTZ DEFAULT NOW(),
    updated_at        TIMESTAMPTZ
);

CREATE INDEX idx_networks_name ON networks (name);

-- Stations
CREATE TABLE IF NOT EXISTS stations (
    station_id          VARCHAR(32) PRIMARY KEY,
    network_id          VARCHAR(32) NOT NULL REFERENCES networks(network_id) ON DELETE CASCADE,
    name                VARCHAR(255) NOT NULL,
    address             TEXT,
    latitude            DOUBLE PRECISION,
    longitude           DOUBLE PRECISION,
    tags                JSONB,
    operational_status  VARCHAR(50) NOT NULL CHECK (
                            operational_status IN ('ACTIVE', 'MAINTENANCE', 'OUT_OF_SERVICE', 'COMMISSIONING')
                        ),
    verification_status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (
                            verification_status IN ('PENDING', 'VERIFIED', 'REJECTED')
                        ),
    is_live             BOOLEAN DEFAULT TRUE,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_stations_network ON stations (network_id);
CREATE INDEX idx_stations_name ON stations (name);

-- Charger Models
CREATE TABLE IF NOT EXISTS charger_models (
    model_id            VARCHAR(32) PRIMARY KEY,
    manufacturer        VARCHAR(255),
    model_name          VARCHAR(255),
    capabilities        JSONB,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

-- Chargers
CREATE TABLE IF NOT EXISTS chargers (
    charger_id          VARCHAR(32) PRIMARY KEY,
    station_id          VARCHAR(32) NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    serial_number       VARCHAR(255) UNIQUE,
    model_id            VARCHAR(32) REFERENCES charger_models(model_id) ON DELETE SET NULL,
    max_power_kw        NUMERIC(7,3),
    status              VARCHAR(50) NOT NULL DEFAULT 'AVAILABLE' CHECK (
                            status IN ('AVAILABLE', 'IN_SERVICE', 'FAULTED', 'OFFLINE', 'MAINTENANCE')
                        ),
    last_seen_at        TIMESTAMPTZ,
    tags                JSONB,
    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

CREATE INDEX idx_chargers_station ON chargers (station_id);
CREATE INDEX idx_chargers_status ON chargers (status);

-- Connector Types
CREATE TABLE IF NOT EXISTS connector_types (
    connector_type_id   INT PRIMARY KEY,
    name                VARCHAR(50) NOT NULL UNIQUE,
    description         TEXT,
    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ
);

-- Connectors
CREATE TABLE IF NOT EXISTS connectors (
    connector_id        VARCHAR(32) PRIMARY KEY,
    charger_id          VARCHAR(32) NOT NULL REFERENCES chargers(charger_id) ON DELETE CASCADE,
    station_id          VARCHAR(32) NOT NULL REFERENCES stations(station_id) ON DELETE CASCADE,
    connector_type_id   INT NOT NULL REFERENCES connector_types(connector_type_id),
    connector_index     INT NOT NULL,
    capacity_kw         NUMERIC(7,3),
    max_current_a       INT,
    operational_status  VARCHAR(50) NOT NULL CHECK (
                            operational_status IN ('AVAILABLE', 'CHARGING', 'FAULTY', 'RESERVED', 'OFFLINE')
                        ),
    verification_status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (
                            verification_status IN ('PENDING', 'VERIFIED', 'REJECTED')
                        ),
    tags                JSONB,
    is_live             BOOLEAN DEFAULT TRUE,
    is_active           BOOLEAN DEFAULT TRUE,
    is_verified         BOOLEAN DEFAULT FALSE,
    created_by          VARCHAR(32) NOT NULL,
    updated_by          VARCHAR(32),
    created_at          TIMESTAMPTZ DEFAULT NOW(),
    updated_at          TIMESTAMPTZ,
    CONSTRAINT uq_connector_per_charger UNIQUE (charger_id, connector_index)
);

CREATE INDEX idx_connectors_charger ON connectors (charger_id);
CREATE INDEX idx_connectors_station ON connectors (station_id);

-- Insert default connector types
INSERT INTO connector_types (connector_type_id, name, description, is_live, is_active, is_verified, created_by)
VALUES
    (1, 'TYPE2', 'Standard AC connector widely used in Europe', TRUE, TRUE, TRUE, 'system'),
    (2, 'CCS', 'Combined Charging System (DC fast charging)', TRUE, TRUE, TRUE, 'system'),
    (3, 'CHADEMO', 'DC fast charging protocol used by Japanese EVs', TRUE, TRUE, TRUE, 'system'),
    (4, 'NEMA1450', 'North American AC connector', TRUE, TRUE, FALSE, 'system'),
    (5, 'TESLA', 'Tesla proprietary connector', TRUE, TRUE, FALSE, 'system')
ON CONFLICT (connector_type_id) DO NOTHING;