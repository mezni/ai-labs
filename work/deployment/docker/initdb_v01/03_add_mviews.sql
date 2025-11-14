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

CREATE UNIQUE INDEX IF NOT EXISTS idx_mv_connector_type_stats_unique 
ON mv_connector_type_stats (station_id, connector_type);


-- ==========================================
-- 1. Drop existing objects to start fresh
-- ==========================================

DROP TRIGGER IF EXISTS trg_refresh_stations_views ON stations;
DROP TRIGGER IF EXISTS trg_refresh_connectors_views ON connectors;
DROP TRIGGER IF EXISTS trg_refresh_connector_types_views ON connector_types;
DROP FUNCTION IF EXISTS trg_refresh_charging_station_views CASCADE;
DROP PROCEDURE IF EXISTS refresh_charging_station_views_with_log CASCADE;
DROP PROCEDURE IF EXISTS refresh_charging_station_views CASCADE;
DROP TABLE IF EXISTS mv_refresh_log;

-- ==========================================
-- 2. Create refresh log table for monitoring
-- ==========================================

CREATE TABLE mv_refresh_log (
    id BIGSERIAL PRIMARY KEY,
    refresh_start TIMESTAMP NOT NULL DEFAULT NOW(),
    refresh_end TIMESTAMP,
    success BOOLEAN,
    error_message TEXT,
    views_refreshed TEXT[],
    duration_ms INTEGER
);

-- Index for efficient time-based queries
CREATE INDEX idx_mv_refresh_log_time ON mv_refresh_log (refresh_start);
CREATE INDEX idx_mv_refresh_log_success ON mv_refresh_log (success);

-- ==========================================
-- 3. Enhanced refresh procedure with logging
-- ==========================================

CREATE OR REPLACE PROCEDURE refresh_charging_station_views_with_log()
LANGUAGE plpgsql
AS $$
DECLARE
    log_id BIGINT;
    refresh_start TIMESTAMP := clock_timestamp();
    refresh_duration INTERVAL;
    refreshed_views TEXT[] := '{}';
BEGIN
    -- Log the refresh attempt
    INSERT INTO mv_refresh_log (refresh_start, views_refreshed) 
    VALUES (refresh_start, ARRAY['mv_connector_type_stats', 'mv_charging_stations_summary', 'mv_charging_stations_geo'])
    RETURNING id INTO log_id;
    
    RAISE NOTICE 'Starting materialized view refresh at % (log_id: %)', refresh_start, log_id;
    
    BEGIN
        -- Refresh views in optimal order (smallest/least complex first)
        RAISE NOTICE 'Refreshing mv_connector_type_stats...';
        REFRESH MATERIALIZED VIEW CONCURRENTLY mv_connector_type_stats;
        refreshed_views := array_append(refreshed_views, 'mv_connector_type_stats');
        
        RAISE NOTICE 'Refreshing mv_charging_stations_summary...';
        REFRESH MATERIALIZED VIEW CONCURRENTLY mv_charging_stations_summary;
        refreshed_views := array_append(refreshed_views, 'mv_charging_stations_summary');
        
        RAISE NOTICE 'Refreshing mv_charging_stations_geo...';
        REFRESH MATERIALIZED VIEW CONCURRENTLY mv_charging_stations_geo;
        refreshed_views := array_append(refreshed_views, 'mv_charging_stations_geo');
        
        -- Calculate duration
        refresh_duration := clock_timestamp() - refresh_start;
        
        -- Log success
        UPDATE mv_refresh_log 
        SET refresh_end = clock_timestamp(),
            success = true,
            views_refreshed = refreshed_views,
            duration_ms = EXTRACT(EPOCH FROM refresh_duration) * 1000
        WHERE id = log_id;
        
        RAISE NOTICE 'Materialized views refreshed successfully in % ms', 
            EXTRACT(EPOCH FROM refresh_duration) * 1000;
            
    EXCEPTION
        WHEN OTHERS THEN
            -- Calculate duration even on failure
            refresh_duration := clock_timestamp() - refresh_start;
            
            -- Log failure with detailed error
            UPDATE mv_refresh_log 
            SET refresh_end = clock_timestamp(),
                success = false,
                error_message = SQLERRM,
                views_refreshed = refreshed_views,
                duration_ms = EXTRACT(EPOCH FROM refresh_duration) * 1000
            WHERE id = log_id;
            
            RAISE WARNING 'Failed to refresh materialized views after % ms: %', 
                EXTRACT(EPOCH FROM refresh_duration) * 1000, SQLERRM;
            
            -- Re-raise the exception to alert calling code
            RAISE;
    END;
END;
$$;

-- ==========================================
-- 4. Simplified refresh procedure (backward compatibility)
-- ==========================================

CREATE OR REPLACE PROCEDURE refresh_charging_station_views()
LANGUAGE plpgsql
AS $$
BEGIN
    CALL refresh_charging_station_views_with_log();
END;
$$;

-- ==========================================
-- 5. Smart trigger function with debouncing
-- ==========================================

CREATE OR REPLACE FUNCTION trg_refresh_charging_station_views()
RETURNS TRIGGER AS $$
DECLARE
    last_successful_refresh TIMESTAMP;
    min_refresh_interval INTERVAL := '1 minutes'; -- Adjust based on your needs
    pending_refresh_count INTEGER;
BEGIN
    -- Check if we already have a recent successful refresh
    SELECT MAX(refresh_start) INTO last_successful_refresh
    FROM mv_refresh_log 
    WHERE success = true;
    
    -- Check if there are pending refreshes in the last minute
    SELECT COUNT(*) INTO pending_refresh_count
    FROM mv_refresh_log 
    WHERE refresh_start > NOW() - INTERVAL '1 minute'
      AND success IS NULL; -- Still running
    
    -- Only trigger refresh if:
    -- 1. No recent successful refresh OR
    -- 2. No pending refreshes already running
    IF (last_successful_refresh IS NULL OR 
        last_successful_refresh < NOW() - min_refresh_interval) AND
       (pending_refresh_count = 0) THEN
        
        RAISE NOTICE 'Triggering materialized view refresh due to % on table %', 
            TG_OP, TG_TABLE_NAME;
        
        -- Use a background worker or direct call
        -- For immediate execution:
        CALL refresh_charging_station_views_with_log();
        
        -- Alternative: Use PG_BACKGROUND for non-blocking execution (if extension available)
        -- PERFORM PG_BACKGROUND_RESULT_DISCARD(
        --     PG_BACKGROUND_LAUNCH('CALL refresh_charging_station_views_with_log()')
        -- );
        
    ELSE
        RAISE NOTICE 'Skipping refresh: recent refresh at %, pending refreshes: %', 
            last_successful_refresh, pending_refresh_count;
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- 6. Create triggers on all relevant tables
-- ==========================================

CREATE TRIGGER trg_refresh_stations_views
AFTER INSERT OR UPDATE OR DELETE ON stations
FOR EACH STATEMENT EXECUTE FUNCTION trg_refresh_charging_station_views();

CREATE TRIGGER trg_refresh_connectors_views
AFTER INSERT OR UPDATE OR DELETE ON connectors
FOR EACH STATEMENT EXECUTE FUNCTION trg_refresh_charging_station_views();

CREATE TRIGGER trg_refresh_connector_types_views
AFTER INSERT OR UPDATE OR DELETE ON connector_types
FOR EACH STATEMENT EXECUTE FUNCTION trg_refresh_charging_station_views();

-- ==========================================
-- 7. Utility functions for monitoring and management
-- ==========================================

-- Function to get refresh statistics
CREATE OR REPLACE FUNCTION get_mv_refresh_stats(
    lookback_hours INTEGER DEFAULT 24
) RETURNS TABLE (
    total_refreshes BIGINT,
    successful_refreshes BIGINT,
    failed_refreshes BIGINT,
    avg_duration_ms NUMERIC,
    last_refresh TIMESTAMP,
    last_success TIMESTAMP,
    last_failure TIMESTAMP
) LANGUAGE plpgsql AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(*) as total_refreshes,
        COUNT(*) FILTER (WHERE success = true) as successful_refreshes,
        COUNT(*) FILTER (WHERE success = false) as failed_refreshes,
        AVG(duration_ms) as avg_duration_ms,
        MAX(refresh_start) as last_refresh,
        MAX(refresh_start) FILTER (WHERE success = true) as last_success,
        MAX(refresh_start) FILTER (WHERE success = false) as last_failure
    FROM mv_refresh_log
    WHERE refresh_start > NOW() - (lookback_hours || ' hours')::INTERVAL;
END;
$$;

-- Function to force refresh regardless of debouncing
CREATE OR REPLACE PROCEDURE force_refresh_charging_station_views()
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE NOTICE 'Forcing materialized view refresh...';
    CALL refresh_charging_station_views_with_log();
END;
$$;

-- Function to clean up old refresh logs
CREATE OR REPLACE PROCEDURE cleanup_refresh_logs(
    keep_days INTEGER DEFAULT 30
)
LANGUAGE plpgsql
AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM mv_refresh_log 
    WHERE refresh_start < NOW() - (keep_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RAISE NOTICE 'Cleaned up % refresh log records older than % days', 
        deleted_count, keep_days;
END;
$$;

-- ==========================================
-- 8. Create monitoring view
-- ==========================================

CREATE OR REPLACE VIEW mv_refresh_monitoring AS
SELECT 
    rl.id,
    rl.refresh_start,
    rl.refresh_end,
    rl.success,
    rl.duration_ms,
    rl.views_refreshed,
    rl.error_message,
    CASE 
        WHEN rl.success = true THEN 'SUCCESS'
        WHEN rl.success = false THEN 'FAILED'
        ELSE 'PENDING'
    END as status,
    NOW() - rl.refresh_start as age
FROM mv_refresh_log rl
ORDER BY rl.refresh_start DESC;

-- ==========================================
-- 9. Initial setup - populate the log and do first refresh
-- ==========================================

-- Insert an initial log entry
INSERT INTO mv_refresh_log (refresh_start, success, views_refreshed, duration_ms)
VALUES (NOW() - INTERVAL '1 hour', true, 
        ARRAY['mv_connector_type_stats', 'mv_charging_stations_summary', 'mv_charging_stations_geo'], 
        1000);

-- Perform initial refresh
CALL refresh_charging_station_views_with_log();

-- ==========================================
-- 10. Usage Examples
-- ==========================================

/*
-- Monitor current status
SELECT * FROM mv_refresh_monitoring LIMIT 10;

-- Get statistics
SELECT * FROM get_mv_refresh_stats(24);

-- Force refresh if needed
CALL force_refresh_charging_station_views();

-- Clean up old logs
CALL cleanup_refresh_logs(30);

-- Check trigger activity
SELECT 
    tgname as trigger_name,
    tgrelid::regclass as table_name,
    tgenabled as enabled
FROM pg_trigger 
WHERE tgname LIKE '%refresh%';
*/