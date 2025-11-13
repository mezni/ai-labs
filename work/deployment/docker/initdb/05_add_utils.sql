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
-- Alternative: More robust version
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
        AND (
            p_connector_types IS NULL 
            OR EXISTS (
                SELECT 1 
                FROM unnest(g.available_connector_names::TEXT[]) AS station_connector
                WHERE station_connector = ANY(p_connector_types)
            )
        )
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