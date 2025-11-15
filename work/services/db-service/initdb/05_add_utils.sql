DROP FUNCTION IF EXISTS find_nearby_stations_detail;

CREATE OR REPLACE FUNCTION find_nearby_stations_detail(
    p_longitude double precision,
    p_latitude double precision,
    p_radius_km double precision DEFAULT 10,
    p_min_power_kw double precision DEFAULT NULL,
    p_connector_types TEXT[] DEFAULT NULL,
    p_power_tiers TEXT[] DEFAULT NULL,
    p_limit INTEGER DEFAULT 50,
    p_offset INTEGER DEFAULT 0
) RETURNS TABLE(
    station_id integer,
    network_id integer,
    network_name varchar,
    network_type varchar,
    contact_email varchar,
    phone_number varchar,
    network_address text,
    station_name varchar,
    station_address text,
    city varchar,
    state varchar,
    country varchar,
    postal_code varchar,
    longitude double precision,
    latitude double precision,
    tags text[],
    station_status varchar,
    is_operational boolean,
    has_available_connectors boolean,
    total_connectors integer,
    total_available_connectors integer,
    total_power_capacity_kw double precision,
    available_power_capacity_kw double precision,
    available_connector_names text[],
    connectors jsonb,
    network jsonb,
    opening_hours text,
    capacity integer,
    fee text,
    parking_fee text,
    access text
) AS $$
DECLARE
    v_radius_meters double precision;
BEGIN
    v_radius_meters := p_radius_km * 1000;
    
    RETURN QUERY
    SELECT 
        s.station_id::integer,
        s.network_id::integer,
        s.network_name::varchar,
        s.network_type::varchar,
        s.contact_email::varchar,
        s.phone_number::varchar,
        s.network_address::text,
        s.station_name::varchar,
        s.station_address::text,
        s.city::varchar,
        s.state::varchar,
        s.country::varchar,
        s.postal_code::varchar,
        s.longitude::double precision,
        s.latitude::double precision,
        s.tags::text[],
        s.station_status::varchar,
        s.is_operational::boolean,
        s.has_available_connectors::boolean,
        s.total_connectors::integer,
        s.total_available_connectors::integer,
        s.total_power_capacity_kw::double precision,
        s.available_power_capacity_kw::double precision,
        s.available_connector_names::text[],
        s.connectors::jsonb,
        s.network::jsonb,
        s.opening_hours::text,
        s.capacity::integer,
        s.fee::text,
        s.parking_fee::text,
        s.access::text
    FROM mv_charging_stations s
    WHERE 
        ST_DWithin(
            ST_SetSRID(ST_MakePoint(s.longitude, s.latitude), 4326)::geography,
            ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)::geography,
            v_radius_meters
        )
    AND (p_min_power_kw IS NULL OR s.available_power_capacity_kw >= p_min_power_kw)
    AND (p_connector_types IS NULL OR s.available_connector_names && p_connector_types)
    AND (p_power_tiers IS NULL OR EXISTS (
        SELECT 1 
        FROM jsonb_array_elements(s.connectors) AS connector
        WHERE (connector->>'power_level_kw')::double precision >= 
            CASE 
                WHEN 'ultra_fast' = ANY(p_power_tiers) THEN 150
                WHEN 'fast' = ANY(p_power_tiers) THEN 50
                WHEN 'medium' = ANY(p_power_tiers) THEN 22
                WHEN 'slow' = ANY(p_power_tiers) THEN 7
                ELSE 0
            END
    ))
    ORDER BY ST_Distance(
        ST_SetSRID(ST_MakePoint(s.longitude, s.latitude), 4326)::geography,
        ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)::geography
    )
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;



DROP FUNCTION IF EXISTS find_nearby_stations;

CREATE OR REPLACE FUNCTION find_nearby_stations(
    p_longitude double precision,
    p_latitude double precision,
    p_radius_km double precision DEFAULT 10,
    p_limit INTEGER DEFAULT 50,
    p_offset INTEGER DEFAULT 0
) RETURNS TABLE(
    station_id integer,
    network_id integer,
    network_name varchar,
    phone_number varchar,
    station_name varchar,
    station_address text,
    city varchar,
    state varchar,
    country varchar,
    longitude double precision,
    latitude double precision,
    tags text[],
    station_status varchar,
    is_operational boolean,
    has_available_connectors boolean,
    total_connectors integer,
    total_available_connectors integer,
    total_power_capacity_kw double precision,
    available_power_capacity_kw double precision,
    available_connector_names text[],
    network jsonb,
    opening_hours text,
    capacity integer,
    fee text,
    parking_fee text,
    access text
) AS $$
DECLARE
    v_radius_meters double precision;
BEGIN
    v_radius_meters := p_radius_km * 1000;
    
    RETURN QUERY
    SELECT 
        s.station_id::integer,
        s.network_id::integer,
        s.network_name::varchar,
        s.phone_number::varchar,
        s.station_name::varchar,
        s.station_address::text,
        s.city::varchar,
        s.state::varchar,
        s.country::varchar,
        s.longitude::double precision,
        s.latitude::double precision,
        s.tags::text[],
        s.station_status::varchar,
        s.is_operational::boolean,
        s.has_available_connectors::boolean,
        s.total_connectors::integer,
        s.total_available_connectors::integer,
        s.total_power_capacity_kw::double precision,
        s.available_power_capacity_kw::double precision,
        s.available_connector_names::text[],
        s.network::jsonb,
        s.opening_hours::text,
        s.capacity::integer,
        s.fee::text,
        s.parking_fee::text,
        s.access::text
    FROM mv_charging_stations s
    WHERE 
        ST_DWithin(
            ST_SetSRID(ST_MakePoint(s.longitude, s.latitude), 4326)::geography,
            ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)::geography,
            v_radius_meters
        )
    ORDER BY ST_Distance(
        ST_SetSRID(ST_MakePoint(s.longitude, s.latitude), 4326)::geography,
        ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)::geography
    )
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;