-- ==========================================
-- Drop existing materialized views
DROP MATERIALIZED VIEW IF EXISTS mv_charging_stations_geo;
DROP MATERIALIZED VIEW IF EXISTS mv_charging_stations_summary;
DROP MATERIALIZED VIEW IF EXISTS mv_connector_type_stats;

-- ==========================================
-- Materialized View: Geographic Charging Stations
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
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    
    -- Quick availability summary
    EXISTS (
        SELECT 1 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS has_available_connectors,
    
    -- Connector summary as JSON
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
        WHERE c.station_id = s.station_id
    ) AS total_connectors,
    
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS total_available_connectors,
    
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
    
    s.tags,
    s.created_by,
    s.updated_by,
    s.created_at,
    s.updated_at
FROM stations s
WHERE s.location IS NOT NULL
WITH DATA;

-- ==========================================
-- Materialized View: Charging Stations Summary
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
    
    COUNT(c.connector_id) AS total_connectors,
    SUM(CASE WHEN c.status = 'available' THEN 1 ELSE 0 END) AS available_connectors,
    SUM(c.power_level_kw) AS total_power,
    MAX(c.power_level_kw) AS max_power_kw,
    MIN(c.power_level_kw) FILTER (WHERE c.power_level_kw > 0) AS min_power_kw,
    AVG(c.power_level_kw) AS avg_power_kw,
    
    COUNT(DISTINCT c.connector_type_id) AS unique_connector_types,
    
    ARRAY_AGG(DISTINCT ct.name) FILTER (WHERE ct.name IS NOT NULL) AS connector_type_names,
    
    EXISTS (
        SELECT 1 
        FROM connectors c2 
        WHERE c2.station_id = s.station_id AND c2.status = 'available'
    ) AS has_available_connectors,
    
    CASE 
        WHEN MAX(c.power_level_kw) >= 150 THEN 'ultra_fast'
        WHEN MAX(c.power_level_kw) >= 50 THEN 'fast'
        WHEN MAX(c.power_level_kw) >= 22 THEN 'medium'
        ELSE 'slow'
    END AS power_tier

FROM stations s
LEFT JOIN connectors c ON s.station_id = c.station_id
LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
GROUP BY s.station_id, s.osm_id, s.name, s.address, s.city, s.state, s.country, s.postal_code, s.location, s.tags, s.created_at, s.updated_at
WITH DATA;

-- ==========================================
-- Materialized View: Connector Type Statistics
CREATE MATERIALIZED VIEW mv_connector_type_stats AS
SELECT 
    s.station_id,
    s.name AS station_name,
    ct.name AS connector_type,
    COUNT(c.connector_id) AS connector_count,
    SUM(CASE WHEN c.status='available' THEN 1 ELSE 0 END) AS available_count,
    SUM(c.power_level_kw) AS total_power_kw,
    AVG(c.power_level_kw) AS avg_power_kw,
    MIN(c.power_level_kw) AS min_power_kw,
    MAX(c.power_level_kw) AS max_power_kw,
    s.location
FROM stations s
JOIN connectors c ON s.station_id=c.station_id
JOIN connector_types ct ON c.connector_type_id=ct.connector_type_id
GROUP BY s.station_id, s.name, ct.name, s.location
WITH DATA;

-- ==========================================
-- Indexes for Materialized Views
CREATE UNIQUE INDEX mv_geo_uidx ON mv_charging_stations_geo(station_id);
CREATE INDEX mv_geo_location_gist ON mv_charging_stations_geo USING GIST(location);
CREATE INDEX mv_geo_available ON mv_charging_stations_geo(has_available_connectors);
CREATE INDEX mv_geo_max_power ON mv_charging_stations_geo(max_power_kw);
CREATE INDEX mv_geo_power_tier ON mv_charging_stations_geo(power_tier);
CREATE INDEX mv_geo_connector_types ON mv_charging_stations_geo USING GIN(available_connector_type_ids);

CREATE UNIQUE INDEX mv_summary_uidx ON mv_charging_stations_summary(station_id);
CREATE INDEX mv_summary_power_tier ON mv_charging_stations_summary(power_tier);

CREATE UNIQUE INDEX mv_connector_type_stats_uidx ON mv_connector_type_stats(station_id, connector_type);
CREATE INDEX mv_connector_stats_station ON mv_connector_type_stats(station_id);
CREATE INDEX mv_connector_stats_type ON mv_connector_type_stats(connector_type);