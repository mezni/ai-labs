-- Drop all existing functions
DROP FUNCTION IF EXISTS find_nearby_stations;
DROP FUNCTION IF EXISTS find_nearby_stations_detail;
DROP FUNCTION IF EXISTS get_station_details;

-- ==========================================
-- Function: Find nearby stations (using CTE)
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
    WITH station_stats AS (
        SELECT 
            s.station_id,
            s.name,
            s.address,
            s.city,
            s.location,
            COALESCE(MAX(c.power_level_kw), 0) as max_power,
            COUNT(CASE WHEN c.status = 'available' THEN 1 END) as available_count,
            COUNT(c.connector_id) as total_count,
            ARRAY_AGG(DISTINCT ct.name) FILTER (WHERE c.status = 'available') as connector_names
        FROM stations s
        LEFT JOIN connectors c ON s.station_id = c.station_id
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE s.location IS NOT NULL
        GROUP BY s.station_id, s.name, s.address, s.city, s.location
    )
    SELECT 
        ss.station_id,
        ss.name::TEXT,
        ss.address::TEXT,
        COALESCE(ss.city, '')::TEXT,
        ST_Distance(ss.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)) / 1000 as distance_km,
        ss.max_power::FLOAT as max_power_kw,
        COALESCE(ss.available_count, 0)::INTEGER as available_connectors,
        COALESCE(ss.total_count, 0)::INTEGER as total_connectors,
        COALESCE(ss.connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        CASE 
            WHEN ss.max_power >= 150 THEN 'ultra_fast'::TEXT
            WHEN ss.max_power >= 50 THEN 'fast'::TEXT
            WHEN ss.max_power >= 22 THEN 'medium'::TEXT
            ELSE 'slow'::TEXT
        END as power_tier,
        TRUE::BOOLEAN as is_operational,
        ST_Y(ss.location::geometry)::FLOAT as latitude,
        ST_X(ss.location::geometry)::FLOAT as longitude
    FROM station_stats ss
    WHERE 
        ST_DWithin(ss.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326), p_radius_km * 1000)
        AND ss.available_count > 0
    ORDER BY distance_km
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Find nearby stations with detailed filtering
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
    WITH station_stats AS (
        SELECT 
            s.station_id,
            s.name,
            s.address,
            s.city,
            s.location,
            COALESCE(MAX(c.power_level_kw), 0) as max_power,
            COUNT(CASE WHEN c.status = 'available' THEN 1 END) as available_count,
            COUNT(c.connector_id) as total_count,
            ARRAY_AGG(DISTINCT ct.name) FILTER (WHERE c.status = 'available') as connector_names,
            CASE 
                WHEN MAX(c.power_level_kw) >= 150 THEN 'ultra_fast'::TEXT
                WHEN MAX(c.power_level_kw) >= 50 THEN 'fast'::TEXT
                WHEN MAX(c.power_level_kw) >= 22 THEN 'medium'::TEXT
                ELSE 'slow'::TEXT
            END as power_tier_calc
        FROM stations s
        LEFT JOIN connectors c ON s.station_id = c.station_id
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE s.location IS NOT NULL
        GROUP BY s.station_id, s.name, s.address, s.city, s.location
    )
    SELECT 
        ss.station_id,
        ss.name::TEXT,
        ss.address::TEXT,
        COALESCE(ss.city, '')::TEXT,
        ST_Distance(ss.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)) / 1000 as distance_km,
        ss.max_power::FLOAT as max_power_kw,
        COALESCE(ss.available_count, 0)::INTEGER as available_connectors,
        COALESCE(ss.total_count, 0)::INTEGER as total_connectors,
        COALESCE(ss.connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        COALESCE(ss.power_tier_calc, 'unknown'::TEXT) as power_tier,
        TRUE::BOOLEAN as is_operational,
        ST_Y(ss.location::geometry)::FLOAT as latitude,
        ST_X(ss.location::geometry)::FLOAT as longitude
    FROM station_stats ss
    WHERE 
        ST_DWithin(ss.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326), p_radius_km * 1000)
        AND ss.available_count > 0
        AND (p_min_power_kw IS NULL OR ss.max_power >= p_min_power_kw)
        AND (p_connector_types IS NULL OR ss.connector_names && p_connector_types)
        AND (p_power_tiers IS NULL OR ss.power_tier_calc = ANY(p_power_tiers))
    ORDER BY distance_km
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function: Get station details by ID
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
    WITH station_stats AS (
        SELECT 
            s.station_id,
            s.name,
            s.address,
            s.city,
            s.state,
            s.country,
            s.postal_code,
            s.location,
            s.tags,
            s.network_id,
            COALESCE(MAX(c.power_level_kw), 0) as max_power,
            COUNT(CASE WHEN c.status = 'available' THEN 1 END) as available_count,
            COUNT(c.connector_id) as total_count,
            ARRAY_AGG(DISTINCT ct.name) FILTER (WHERE c.status = 'available') as connector_names,
            jsonb_agg(
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
            ) as connectors_json
        FROM stations s
        LEFT JOIN connectors c ON s.station_id = c.station_id
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE s.station_id = p_station_id
        GROUP BY s.station_id, s.name, s.address, s.city, s.state, s.country, s.postal_code, s.location, s.tags, s.network_id
    )
    SELECT 
        ss.station_id,
        ss.name::TEXT,
        ss.address::TEXT,
        COALESCE(ss.city, '')::TEXT,
        COALESCE(ss.state, '')::TEXT,
        COALESCE(ss.country, '')::TEXT,
        COALESCE(ss.postal_code, '')::TEXT,
        ST_Y(ss.location::geometry)::FLOAT as latitude,
        ST_X(ss.location::geometry)::FLOAT as longitude,
        ss.max_power::FLOAT as max_power_kw,
        COALESCE(ss.available_count, 0)::INTEGER as available_connectors,
        COALESCE(ss.total_count, 0)::INTEGER as total_connectors,
        COALESCE(ss.connector_names, ARRAY[]::TEXT[])::TEXT[] as connector_types,
        CASE 
            WHEN ss.max_power >= 150 THEN 'ultra_fast'::TEXT
            WHEN ss.max_power >= 50 THEN 'fast'::TEXT
            WHEN ss.max_power >= 22 THEN 'medium'::TEXT
            ELSE 'slow'::TEXT
        END as power_tier,
        COALESCE(ss.connectors_json, '[]'::JSONB) as connectors,
        COALESCE(hstore_to_json(ss.tags), '{}'::JSONB) as tags,
        COALESCE(n.name, 'Unknown')::TEXT as network_name,
        TRUE::BOOLEAN as is_operational
    FROM station_stats ss
    LEFT JOIN networks n ON ss.network_id = n.network_id;
END;
$$ LANGUAGE plpgsql;