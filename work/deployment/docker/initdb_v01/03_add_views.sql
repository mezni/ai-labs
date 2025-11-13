-- Drop existing views
DROP VIEW IF EXISTS stations_summary CASCADE;
DROP VIEW IF EXISTS stations_details CASCADE;

-- Fixed stations_summary view
CREATE OR REPLACE VIEW stations_summary AS
SELECT 
    s.station_id,
    s.network_id,
    n.name AS network_name,
    s.name AS station_name,
    s.address,
    s.city,
    s.state,  -- Added missing column
    s.country,  -- Added missing column
    s.postal_code,
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    s.location,
    s.tags,
    s.status AS station_status,
    s.is_operational,
    
    -- Connector statistics
    COUNT(c.connector_id) AS total_connectors,
    COUNT(CASE WHEN c.status = 'available' THEN 1 END) AS available_connectors,
    COUNT(CASE WHEN c.status = 'occupied' THEN 1 END) AS occupied_connectors,
    COUNT(CASE WHEN c.status = 'out_of_service' THEN 1 END) AS out_of_service_connectors,
    COUNT(CASE WHEN c.status = 'reserved' THEN 1 END) AS reserved_connectors,
    
    -- Power statistics
    MAX(c.power_level_kw) AS max_power_kw,
    MIN(c.power_level_kw) AS min_power_kw,
    SUM(c.power_level_kw) AS total_power_kw,
    
    -- Connector type breakdown
    COUNT(DISTINCT c.connector_type_id) AS unique_connector_types,
    COUNT(CASE WHEN ct.current_type = 'DC' THEN 1 END) AS dc_connectors,
    COUNT(CASE WHEN ct.current_type = 'AC' THEN 1 END) AS ac_connectors,
    
    -- Availability status
    CASE 
        WHEN COUNT(CASE WHEN c.status = 'available' THEN 1 END) > 0 THEN 'available'
        WHEN COUNT(c.connector_id) = 0 THEN 'no_connectors'
        ELSE 'busy'
    END AS availability_status

FROM stations s
LEFT JOIN networks n ON s.network_id = n.network_id
LEFT JOIN connectors c ON s.station_id = c.station_id
LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
WHERE s.is_operational = TRUE
GROUP BY 
    s.station_id, n.network_id, n.name, s.name, s.address, 
    s.city, s.state, s.country, s.postal_code, s.location, 
    s.tags, s.status, s.is_operational;

-- Drop existing view if it exists
DROP VIEW IF EXISTS stations_details CASCADE;

-- Create comprehensive stations detailed view
CREATE OR REPLACE VIEW stations_details AS
WITH station_connectors AS (
    SELECT 
        c.station_id,
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'connector_id', c.connector_id,
                'connector_type_id', c.connector_type_id,
                'connector_type_name', ct.name,
                'connector_type_description', ct.description,
                'standard', ct.standard,
                'current_type', ct.current_type,
                'typical_power_kw', ct.typical_power_kw,
                'power_level_kw', c.power_level_kw,
                'status', c.status,
                'max_voltage', c.max_voltage,
                'max_amperage', c.max_amperage,
                'serial_number', c.serial_number,
                'manufacturer', c.manufacturer,
                'model', c.model,
                'installation_date', c.installation_date,
                'last_maintenance_date', c.last_maintenance_date,
                'created_at', c.created_at,
                'updated_at', c.updated_at
            )
            ORDER BY 
                ct.current_type DESC,  -- DC first, then AC
                c.power_level_kw DESC,
                ct.name
        ) AS connectors,
        
        -- Connector statistics
        COUNT(*) AS total_connectors,
        COUNT(CASE WHEN c.status = 'available' THEN 1 END) AS available_connectors,
        COUNT(CASE WHEN c.status = 'occupied' THEN 1 END) AS occupied_connectors,
        COUNT(CASE WHEN c.status = 'out_of_service' THEN 1 END) AS out_of_service_connectors,
        COUNT(CASE WHEN c.status = 'reserved' THEN 1 END) AS reserved_connectors,
        COUNT(CASE WHEN c.status = 'unavailable' THEN 1 END) AS unavailable_connectors,
        
        -- Power statistics
        MAX(c.power_level_kw) AS max_power_kw,
        MIN(c.power_level_kw) AS min_power_kw,
        ROUND(AVG(c.power_level_kw), 2) AS avg_power_kw,
        SUM(c.power_level_kw) AS total_power_kw,
        
        -- Connector type breakdown
        COUNT(DISTINCT c.connector_type_id) AS unique_connector_types,
        COUNT(CASE WHEN ct.current_type = 'DC' THEN 1 END) AS dc_connectors,
        COUNT(CASE WHEN ct.current_type = 'AC' THEN 1 END) AS ac_connectors,
        
        -- Fast charging capability (DC >= 50kW)
        COUNT(CASE WHEN ct.current_type = 'DC' AND c.power_level_kw >= 50 THEN 1 END) AS fast_charging_connectors,
        
        -- Connector type list
        ARRAY_AGG(DISTINCT ct.name) AS connector_type_names,
        ARRAY_AGG(DISTINCT ct.connector_type_id) AS connector_type_ids
    FROM connectors c
    JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
    GROUP BY c.station_id
),
station_verification_aggregated AS (
    SELECT 
        station_id,
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'verification_id', station_verification_history_id,
                'status', status,
                'updated_by', updated_by,
                'updated_at', updated_at,
                'notes', notes
            )
            ORDER BY updated_at DESC
        ) AS verification_history
    FROM station_verification_history
    GROUP BY station_id
),
station_verification_latest AS (
    SELECT DISTINCT ON (station_id)
        station_id,
        status AS latest_verification_status,
        updated_at
    FROM station_verification_history
    ORDER BY station_id, updated_at DESC
)
SELECT 
    -- Station Basic Information
    s.station_id,
    s.network_id,
    n.name AS network_name,
    n.type AS network_type,
    n.contact_email AS network_contact_email,
    n.phone_number AS network_phone,
    n.address AS network_address,
    
    -- Company Information (if available)
    co.company_id,
    co.business_registration_number,
    co.website_url,
    
    -- Station Details
    s.name AS station_name,
    s.address AS station_address,
    s.city,
    s.state,
    s.country,
    s.postal_code,
    
    -- Geospatial Information
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    s.location,
    s.osm_id,
    
    -- Station Status and Metadata
    s.status AS station_status,
    s.is_operational,
    s.tags,
    
    -- Tag Extractions
    s.tags -> 'amenity' AS amenity_type,
    COALESCE((s.tags -> 'capacity')::integer, 0) AS capacity,
    s.tags -> 'fee' AS has_fee,
    s.tags -> 'parking_fee' AS has_parking_fee,
    s.tags -> 'access' AS access_type,
    s.tags -> 'operator' AS operator_name,
    s.tags -> 'opening_hours' AS opening_hours,
    s.tags -> 'capacity:disabled' AS disabled_capacity,
    s.tags -> 'socket:type2' AS socket_type2,
    s.tags -> 'socket:chademo' AS socket_chademo,
    s.tags -> 'socket:ccs' AS socket_ccs,
    
    -- Connector Information
    COALESCE(sc.connectors, '[]'::json) AS connectors,
    
    -- Connector Statistics
    COALESCE(sc.total_connectors, 0) AS total_connectors,
    COALESCE(sc.available_connectors, 0) AS available_connectors,
    COALESCE(sc.occupied_connectors, 0) AS occupied_connectors,
    COALESCE(sc.out_of_service_connectors, 0) AS out_of_service_connectors,
    COALESCE(sc.reserved_connectors, 0) AS reserved_connectors,
    COALESCE(sc.unavailable_connectors, 0) AS unavailable_connectors,
    
    -- Power Statistics
    sc.max_power_kw,
    sc.min_power_kw,
    sc.avg_power_kw,
    sc.total_power_kw,
    
    -- Connector Type Information
    COALESCE(sc.unique_connector_types, 0) AS unique_connector_types,
    COALESCE(sc.dc_connectors, 0) AS dc_connectors,
    COALESCE(sc.ac_connectors, 0) AS ac_connectors,
    COALESCE(sc.fast_charging_connectors, 0) AS fast_charging_connectors,
    COALESCE(sc.connector_type_names, ARRAY[]::text[]) AS connector_type_names,
    COALESCE(sc.connector_type_ids, ARRAY[]::integer[]) AS connector_type_ids,
    
    -- Availability Status
    CASE 
        WHEN COALESCE(sc.available_connectors, 0) > 0 THEN 'available'
        WHEN COALESCE(sc.total_connectors, 0) = 0 THEN 'no_connectors'
        ELSE 'busy'
    END AS availability_status,
    
    -- Fast Charging Availability
    CASE 
        WHEN COALESCE(sc.fast_charging_connectors, 0) > 0 THEN 'available'
        WHEN COALESCE(sc.dc_connectors, 0) > 0 THEN 'standard_dc'
        ELSE 'ac_only'
    END AS fast_charging_status,
    
    -- Verification Information
    COALESCE(sva.verification_history, '[]'::json) AS verification_history,
    COALESCE(svl.latest_verification_status, s.status) AS latest_verification_status,
    
    -- Audit Information
    s.created_by,
    s.updated_by,
    s.created_at,
    s.updated_at,
    
    -- Additional computed fields
    CASE 
        WHEN sc.max_power_kw >= 150 THEN 'ultra_fast'
        WHEN sc.max_power_kw >= 50 THEN 'fast'
        ELSE 'standard'
    END AS charging_speed_category,
    
    -- Location completeness score (0-100)
    CASE 
        WHEN s.location IS NOT NULL AND s.address IS NOT NULL AND s.city IS NOT NULL THEN 100
        WHEN s.location IS NOT NULL AND s.address IS NOT NULL THEN 80
        WHEN s.location IS NOT NULL THEN 60
        WHEN s.address IS NOT NULL AND s.city IS NOT NULL THEN 40
        ELSE 20
    END AS location_completeness_score

FROM stations s
LEFT JOIN networks n ON s.network_id = n.network_id
LEFT JOIN companies co ON n.network_id = co.network_id
LEFT JOIN station_connectors sc ON s.station_id = sc.station_id
LEFT JOIN station_verification_aggregated sva ON s.station_id = sva.station_id
LEFT JOIN station_verification_latest svl ON s.station_id = svl.station_id
WHERE s.is_operational = TRUE;