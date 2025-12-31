CREATE TABLE stations (
    -- Primary Identifier (UUID)
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Relationships
    location_id UUID, -- Reference to separate location table
    
    -- Basic Information
    name VARCHAR(100) NOT NULL,
    
    -- Geographic Coordinates (PostGIS Geography type)
    location GEOGRAPHY(Point, 4326) NOT NULL,
    -- Geographic Coordinates (Kept in stations for performance)
    latitude DECIMAL(10, 8) NOT NULL,
    longitude DECIMAL(11, 8) NOT NULL,

    -- Status Management
    status VARCHAR(20) CHECK (status IN ('ONLINE', 'OFFLINE', 'MAINTENANCE', 'DECOMMISSIONED')) NOT NULL DEFAULT 'OFFLINE',
    is_active BOOLEAN DEFAULT TRUE,
    is_verified BOOLEAN DEFAULT FALSE,
    is_live BOOLEAN DEFAULT FALSE,
        
    -- Audit Fields
    created_by VARCHAR(100) NOT NULL,
    updated_by VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

-- Indexes for Stations

-- Spatial Index for Geography column (CRITICAL for performance)
CREATE INDEX idx_stations_location_geography ON stations USING GIST(location);