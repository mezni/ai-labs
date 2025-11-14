-- ==========================================
-- Basic Tests for Tunis
-- ==========================================

-- Test 1: Basic nearby search in Tunis
SELECT * FROM find_nearby_stations(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 5,
    p_limit := 10
);

-- Test 2: Detailed search in Tunis center
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 3,
    p_limit := 10
);

-- ==========================================
-- Different Areas in Tunis
-- ==========================================

-- Tunis Medina (old city)
SELECT * FROM find_nearby_stations(10.1715, 36.7965, 2, 10);

-- Lac de Tunis area  
SELECT * FROM find_nearby_stations(10.2415, 36.8365, 3, 10);

-- Carthage area (historical site)
SELECT * FROM find_nearby_stations(10.3215, 36.8515, 2, 10);

-- Sidi Bou Said (famous blue village)
SELECT * FROM find_nearby_stations(10.3415, 36.8715, 2, 10);

-- ==========================================
-- Filtered Searches in Tunis
-- ==========================================

-- Fast chargers only in Tunis
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 10,
    p_min_power_kw := 50,
    p_limit := 10
);

-- Specific connector types in Tunis
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 10,
    p_connector_types := ARRAY['Type 2', 'CCS Combo 1'],
    p_limit := 10
);

-- Ultra-fast chargers only
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 15,
    p_min_power_kw := 100,
    p_power_tiers := ARRAY['ultra_fast', 'fast'],
    p_limit := 10
);

-- ==========================================
-- Different Radius Sizes around Tunis
-- ==========================================

-- City center only (small radius)
SELECT * FROM find_nearby_stations(10.1815, 36.8065, 1, 5);

-- Greater Tunis area
SELECT * FROM find_nearby_stations(10.1815, 36.8065, 10, 15);

-- Large area covering suburbs
SELECT * FROM find_nearby_stations(10.1815, 36.8065, 20, 20);

-- ==========================================
-- Real-World Scenarios in Tunis
-- ==========================================

-- Airport charging (Tunis-Carthage Airport)
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.2272,
    p_latitude := 36.8513,
    p_radius_km := 2,
    p_min_power_kw := 50,
    p_limit := 5
);

-- Hotel charging in city center
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 1,
    p_limit := 5
);

-- Highway charging near Tunis
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.0815,
    p_latitude := 36.8065,
    p_radius_km := 5,
    p_min_power_kw := 100,
    p_limit := 5
);

-- ==========================================
-- Edge Cases around Tunis
-- ==========================================

-- Very small radius (should show only closest stations)
SELECT * FROM find_nearby_stations(10.1815, 36.8065, 0.1, 3);

-- No filters (get all nearby stations)
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 5,
    p_min_power_kw := NULL,
    p_connector_types := NULL,
    p_power_tiers := NULL,
    p_limit := 10
);

-- Only AC chargers (Type 2)
SELECT * FROM find_nearby_stations_detail(
    p_longitude := 10.1815,
    p_latitude := 36.8065,
    p_radius_km := 5,
    p_connector_types := ARRAY['Type 2'],
    p_limit := 10
);

# Coordinates for Tunis, Tunisia (city center)
LONGITUDE=10.1815
LATITUDE=36.8065

# Get nearby stations in Tunis (basic)
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=20&limit=10"

# Get nearby stations with custom radius (10km around Tunis)
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=10&limit=20&offset=0"

# Get detailed nearby stations with filtering in Tunis
curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=10&limit=10"

# Get detailed with connector type filtering in Tunis
curl "http://localhost:8080/api/stations/nearby/detailed?longitude=10.1815&latitude=36.8065&connector_types=type2,ccs&power_tiers=medium,fast"

# Get detailed with all filters in Tunis
curl "http://localhost:8080/api/stations/nearby/detailed?longitude=10.1815&latitude=36.8065&radius_km=15&min_power_kw=50&connector_types=ccs&power_tiers=fast,ultra_fast&limit=10&offset=0"






curl "http://localhost:8080/api/stations/nearby?longitude=10.1815&latitude=36.8065&radius_km=10&limit=10"|jq .
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  1430  100  1430    0     0   271k      0 --:--:-- --:--:-- --:--:--  279k
{
  "success": true,
  "data": [
    {
      "station_id": 2,
      "name": "Hotel Golden Tulip El Mechtel",
      "address": "Avenue Ouled Haffouz, Tunis",
      "city": "Tunis",
      "distance_km": 4.20093906095,
      "max_power_kw": 200.0,
      "available_connectors": 1,
      "total_connectors": 2,
      "connector_types": [
        "Type 2 (Mennekes)"
      ],
      "power_tier": "ultra_fast",
      "is_operational": true,
      "latitude": 36.8374,
      "longitude": 10.2087
    },
    {
      "station_id": 7,
      "name": "Aeroport Tunis-Carthage",
      "address": "Aéroport International de Tunis-Carthage",
      "city": "Tunis",
      "distance_km": 6.4038146960499995,
      "max_power_kw": 350.0,
      "available_connectors": 2,
      "total_connectors": 2,
      "connector_types": [
        "CCS2 (Combo 2)",
        "Type 2 (Mennekes)"
      ],
      "power_tier": "ultra_fast",
      "is_operational": true,
      "latitude": 36.851,
      "longitude": 10.2272
    },
    {
      "station_id": 1,
      "name": "STEG Charging Station - Lac",
      "address": "Lac de Tunis, near Tunis City Center",
      "city": "Tunis",
      "distance_km": 6.408350535339999,
      "max_power_kw": 350.0,
      "available_connectors": 2,
      "total_connectors": 2,
      "connector_types": [
        "CCS2 (Combo 2)",
        "Type 2 (Mennekes)"
      ],
      "power_tier": "ultra_fast",
      "is_operational": true,
      "latitude": 36.838,
      "longitude": 10.2417
    },
    {
      "station_id": 3,
      "name": "Tunisia Mall Charging Point",
      "address": "Les Berges du Lac, Tunis",
      "city": "Tunis",
      "distance_km": 7.03098518825,
      "max_power_kw": 350.0,
      "available_connectors": 2,
      "total_connectors": 3,
      "connector_types": [
        "CHAdeMO",
        "Type 2 (Mennekes)"
      ],
      "power_tier": "ultra_fast",
      "is_operational": true,
      "latitude": 36.851,
      "longitude": 10.2376
    }
  ],
  "error": null
}
























-- Drop existing materialized views
DROP MATERIALIZED VIEW IF EXISTS mv_charging_stations_geo;
DROP MATERIALIZED VIEW IF EXISTS mv_charging_stations_summary;
DROP MATERIALIZED VIEW IF EXISTS mv_connector_type_stats;

-- Main geographic optimized view
CREATE MATERIALIZED VIEW mv_charging_stations_geo AS
SELECT 
    s.station_id,
    s.osm_id,
    s.name,
    s.address,
    s.city,
    s.state,
    s.country,
    s.postal_code,
    s.location,
    
    -- Extract coordinates for easy access
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    
    -- Quick availability summary
    EXISTS (
        SELECT 1 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS has_available_connectors,
    
    -- Connector summary as JSON for fast retrieval
    (
        SELECT jsonb_agg(
            jsonb_build_object(
                'connector_id', c.connector_id,
                'type_id', c.connector_type_id,
                'type_name', ct.name,
                'status', c.status,
                'power_level_kw', c.power_level_kw,
                'max_voltage', c.max_voltage,
                'max_amperage', c.max_amperage,
                'manufacturer', c.manufacturer,
                'model', c.model
            ) ORDER BY c.power_level_kw DESC NULLS LAST
        )
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id
    ) AS connectors,
    
    -- Power statistics
    (
        SELECT MAX(power_level_kw) 
        FROM connectors c 
        WHERE c.station_id = s.station_id
    ) AS max_power_kw,
    
    (
        SELECT MIN(power_level_kw) 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.power_level_kw > 0
    ) AS min_power_kw,
    
    -- Connector counts
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS total_available_connectors,
    
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id
    ) AS total_connectors,
    
    -- Available connector types array
    (
        SELECT ARRAY_AGG(DISTINCT c.connector_type_id)
        FROM connectors c
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_connector_type_ids,

    (
        SELECT ARRAY_AGG(DISTINCT ct.name)
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_connector_names,
    
    -- Power tier classification
    CASE 
        WHEN (SELECT MAX(power_level_kw) FROM connectors WHERE station_id = s.station_id) >= 150 THEN 'ultra_fast'
        WHEN (SELECT MAX(power_level_kw) FROM connectors WHERE station_id = s.station_id) >= 50 THEN 'fast'
        WHEN (SELECT MAX(power_level_kw) FROM connectors WHERE station_id = s.station_id) >= 22 THEN 'medium'
        ELSE 'slow'
    END AS power_tier,

    -- Station metadata from tags
    s.tags->'operator' AS operator,
    s.tags->'opening_hours' AS opening_hours,
    s.tags->'capacity' AS capacity,
    s.tags->'fee' AS fee,
    s.tags->'parking_fee' AS parking_fee,
    s.tags->'access' AS access,

    -- Network information
    n.name AS network_name,

    -- Timestamps
    s.created_at,
    s.updated_at

FROM stations s
LEFT JOIN networks n ON s.network_id = n.network_id
WHERE s.location IS NOT NULL
AND s.status = 'verified'  -- Only show verified stations
AND s.is_operational = true
WITH DATA;

-- Summary view for analytics
CREATE MATERIALIZED VIEW mv_charging_stations_summary AS
SELECT 
    s.station_id,
    s.osm_id,
    s.name,
    s.address,
    s.city,
    s.state,
    s.country,
    s.postal_code,
    s.location,
    s.tags,
    s.created_at,
    s.updated_at,
    
    -- Connector statistics
    COUNT(c.connector_id) AS total_connectors,
    COUNT(CASE WHEN c.status = 'available' THEN 1 END) AS available_connectors,
    MAX(c.power_level_kw) AS max_power_kw,
    MIN(c.power_level_kw) AS min_power_kw,
    AVG(c.power_level_kw) AS avg_power_kw,
    
    -- Connector type breakdown
    COUNT(DISTINCT c.connector_type_id) AS unique_connector_types,
    
    -- Array of available connector types
    ARRAY_AGG(DISTINCT ct.name) FILTER (WHERE ct.name IS NOT NULL) AS connector_type_names,
    
    -- Current status summary
    EXISTS (
        SELECT 1 FROM connectors c2 
        WHERE c2.station_id = s.station_id AND c2.status = 'available'
    ) AS has_available_connectors,
    
    -- Power capacity tiers
    CASE 
        WHEN MAX(c.power_level_kw) >= 150 THEN 'ultra_fast'
        WHEN MAX(c.power_level_kw) >= 50 THEN 'fast'
        WHEN MAX(c.power_level_kw) >= 22 THEN 'medium'
        ELSE 'slow'
    END AS power_tier,

    -- Network information
    n.name AS network_name

FROM stations s
LEFT JOIN connectors c ON s.station_id = c.station_id
LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
LEFT JOIN networks n ON s.network_id = n.network_id
WHERE s.status = 'verified'
AND s.is_operational = true
GROUP BY s.station_id, s.osm_id, s.name, s.address, s.city, s.state, s.country, s.postal_code, s.location, s.tags, s.created_at, s.updated_at, n.name
WITH DATA;

-- Connector type statistics view
CREATE MATERIALIZED VIEW mv_connector_type_stats AS
SELECT 
    s.station_id,
    s.name AS station_name,
    ct.name AS connector_type,
    COUNT(c.connector_id) AS connector_count,
    COUNT(CASE WHEN c.status = 'available' THEN 1 END) AS available_count,
    AVG(c.power_level_kw) AS avg_power_kw,
    MIN(c.power_level_kw) AS min_power_kw,
    MAX(c.power_level_kw) AS max_power_kw,
    s.location,
    s.city,
    s.state,
    s.country

FROM stations s
JOIN connectors c ON s.station_id = c.station_id
JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
WHERE s.status = 'verified'
AND s.is_operational = true
GROUP BY s.station_id, s.name, ct.name, s.location, s.city, s.state, s.country
WITH DATA;

-- Create indexes for performance
CREATE UNIQUE INDEX idx_mv_geo_station_id ON mv_charging_stations_geo (station_id);
CREATE INDEX idx_mv_geo_location_gist ON mv_charging_stations_geo USING GIST (location);
CREATE INDEX idx_mv_geo_coords ON mv_charging_stations_geo (longitude, latitude);
CREATE INDEX idx_mv_geo_available ON mv_charging_stations_geo (has_available_connectors);
CREATE INDEX idx_mv_geo_max_power ON mv_charging_stations_geo (max_power_kw);
CREATE INDEX idx_mv_geo_power_tier ON mv_charging_stations_geo (power_tier);
CREATE INDEX idx_mv_geo_operator ON mv_charging_stations_geo (operator);
CREATE INDEX idx_mv_geo_connector_types ON mv_charging_stations_geo USING GIN (available_connector_type_ids);
CREATE INDEX idx_mv_geo_connector_names ON mv_charging_stations_geo USING GIN (available_connector_names);

CREATE UNIQUE INDEX idx_mv_summary_station_id ON mv_charging_stations_summary (station_id);
CREATE INDEX idx_mv_summary_location ON mv_charging_stations_summary USING GIST (location);
CREATE INDEX idx_mv_summary_power_tier ON mv_charging_stations_summary (power_tier);
CREATE INDEX idx_mv_summary_city ON mv_charging_stations_summary (city);
CREATE INDEX idx_mv_summary_country ON mv_charging_stations_summary (country);

CREATE INDEX idx_mv_connector_stats_station ON mv_connector_type_stats (station_id);
CREATE INDEX idx_mv_connector_stats_type ON mv_connector_type_stats (connector_type);
CREATE INDEX idx_mv_connector_stats_location ON mv_connector_type_stats USING GIST (location);



-- Drop and recreate optimized functions using materialized views
DROP FUNCTION IF EXISTS find_nearby_stations;
DROP FUNCTION IF EXISTS find_nearby_stations_detail;
DROP FUNCTION IF EXISTS get_station_details;

-- ==========================================
-- Function: Find nearby stations (optimized using MV)
CREATE OR REPLACE FUNCTION find_nearby_stations(
    p_longitude FLOAT,
    p_latitude FLOAT,
    p_radius_km FLOAT DEFAULT 10,
    p_limit INTEGER DEFAULT 50,
    p_offset INTEGER DEFAULT 0
) RETURNS TABLE(
    station_id INTEGER,
    name TEXT,
    address TEXT,
    city TEXT,
    distance_km FLOAT,
    max_power_kw FLOAT,
    available_connectors INTEGER,
    total_connectors INTEGER,
    connector_types TEXT[],
    power_tier TEXT,
    is_operational BOOLEAN,
    latitude FLOAT,
    longitude FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.station_id,
        g.name::TEXT,
        g.address::TEXT,
        COALESCE(g.city, '')::TEXT,
        ST_Distance(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)) / 1000 as distance_km,
        COALESCE(g.max_power_kw, 0)::FLOAT as max_power_kw,
        COALESCE(g.total_available_connectors, 0)::INTEGER as available_connectors,
        COALESCE(g.total_connectors, 0)::INTEGER as total_connectors,
        COALESCE(g.available_connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        COALESCE(g.power_tier, 'unknown')::TEXT as power_tier,
        TRUE::BOOLEAN as is_operational,
        g.latitude::FLOAT,
        g.longitude::FLOAT
    FROM mv_charging_stations_geo g
    WHERE 
        ST_DWithin(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326), p_radius_km * 1000)
        AND g.has_available_connectors = true
    ORDER BY distance_km
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Find nearby stations with detailed filtering (optimized)
CREATE OR REPLACE FUNCTION find_nearby_stations_detail(
    p_longitude FLOAT,
    p_latitude FLOAT,
    p_radius_km FLOAT DEFAULT 10,
    p_min_power_kw FLOAT DEFAULT NULL,
    p_connector_types TEXT[] DEFAULT NULL,
    p_power_tiers TEXT[] DEFAULT NULL,
    p_limit INTEGER DEFAULT 50,
    p_offset INTEGER DEFAULT 0
) RETURNS TABLE(
    station_id INTEGER,
    name TEXT,
    address TEXT,
    city TEXT,
    distance_km FLOAT,
    max_power_kw FLOAT,
    available_connectors INTEGER,
    total_connectors INTEGER,
    connector_types TEXT[],
    power_tier TEXT,
    is_operational BOOLEAN,
    latitude FLOAT,
    longitude FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.station_id,
        g.name::TEXT,
        g.address::TEXT,
        COALESCE(g.city, '')::TEXT,
        ST_Distance(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)) / 1000 as distance_km,
        COALESCE(g.max_power_kw, 0)::FLOAT as max_power_kw,
        COALESCE(g.total_available_connectors, 0)::INTEGER as available_connectors,
        COALESCE(g.total_connectors, 0)::INTEGER as total_connectors,
        COALESCE(g.available_connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        COALESCE(g.power_tier, 'unknown')::TEXT as power_tier,
        TRUE::BOOLEAN as is_operational,
        g.latitude::FLOAT,
        g.longitude::FLOAT
    FROM mv_charging_stations_geo g
    WHERE 
        ST_DWithin(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326), p_radius_km * 1000)
        AND g.has_available_connectors = true
        AND (p_min_power_kw IS NULL OR g.max_power_kw >= p_min_power_kw)
        AND (p_connector_types IS NULL OR g.available_connector_names && p_connector_types)
        AND (p_power_tiers IS NULL OR g.power_tier = ANY(p_power_tiers))
    ORDER BY distance_km
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Get station details by ID (optimized)
CREATE OR REPLACE FUNCTION get_station_details(p_station_id INTEGER)
RETURNS TABLE(
    station_id INTEGER,
    name TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    country TEXT,
    postal_code TEXT,
    latitude FLOAT,
    longitude FLOAT,
    max_power_kw FLOAT,
    available_connectors INTEGER,
    total_connectors INTEGER,
    connector_types TEXT[],
    power_tier TEXT,
    connectors JSONB,
    tags JSONB,
    network_name TEXT,
    is_operational BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.station_id,
        g.name::TEXT,
        g.address::TEXT,
        COALESCE(g.city, '')::TEXT,
        COALESCE(g.state, '')::TEXT,
        COALESCE(g.country, '')::TEXT,
        COALESCE(g.postal_code, '')::TEXT,
        g.latitude::FLOAT,
        g.longitude::FLOAT,
        COALESCE(g.max_power_kw, 0)::FLOAT as max_power_kw,
        COALESCE(g.total_available_connectors, 0)::INTEGER as available_connectors,
        COALESCE(g.total_connectors, 0)::INTEGER as total_connectors,
        COALESCE(g.available_connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        COALESCE(g.power_tier, 'unknown')::TEXT as power_tier,
        COALESCE(g.connectors, '[]'::JSONB) as connectors,
        COALESCE(hstore_to_json(g.tags), '{}'::JSONB) as tags,
        COALESCE(g.network_name, 'Unknown')::TEXT as network_name,
        TRUE::BOOLEAN as is_operational
    FROM mv_charging_stations_geo g
    WHERE g.station_id = p_station_id;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Export stations as GeoJSON (for maps)
CREATE OR REPLACE FUNCTION export_stations_geojson()
RETURNS JSON AS $$
DECLARE
    result JSON;
BEGIN
    SELECT json_build_object(
        'type', 'FeatureCollection',
        'features', json_agg(
            json_build_object(
                'type', 'Feature',
                'geometry', ST_AsGeoJSON(location::geometry)::json,
                'properties', json_build_object(
                    'station_id', station_id,
                    'name', name,
                    'address', address,
                    'city', city,
                    'max_power_kw', max_power_kw,
                    'available_connectors', total_available_connectors,
                    'total_connectors', total_connectors,
                    'connector_types', available_connector_names,
                    'power_tier', power_tier,
                    'operator', operator,
                    'network_name', network_name
                )
            )
        )
    ) INTO result
    FROM mv_charging_stations_geo;

    RETURN COALESCE(result, '{"type":"FeatureCollection","features":[]}'::json);
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Get system statistics
CREATE OR REPLACE FUNCTION get_system_stats()
RETURNS TABLE(
    total_stations BIGINT,
    total_connectors BIGINT,
    available_connectors BIGINT,
    avg_power_kw NUMERIC,
    stations_with_available BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(*) as total_stations,
        SUM(total_connectors) as total_connectors,
        SUM(total_available_connectors) as available_connectors,
        AVG(max_power_kw) as avg_power_kw,
        COUNT(*) FILTER (WHERE has_available_connectors = true) as stations_with_available
    FROM mv_charging_stations_summary;
END;
$$ LANGUAGE plpgsql;