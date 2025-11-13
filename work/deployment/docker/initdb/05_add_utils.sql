-- ==========================================
-- Refresh function for materialized views
CREATE OR REPLACE FUNCTION refresh_charging_station_views()
RETURNS VOID AS $$
BEGIN
    RAISE NOTICE 'Refreshing materialized views...';
    
    -- Refresh materialized views
    BEGIN
        REFRESH MATERIALIZED VIEW mv_charging_stations_geo;
        REFRESH MATERIALIZED VIEW mv_charging_stations_summary;
        REFRESH MATERIALIZED VIEW mv_connector_type_stats;
        RAISE NOTICE 'Materialized views refreshed successfully';
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'Error refreshing materialized views: %', SQLERRM;
    END;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Immediate refresh trigger function
CREATE OR REPLACE FUNCTION trg_refresh_mv_immediate()
RETURNS TRIGGER AS $$
BEGIN
    -- Use a deferred approach to avoid transaction conflicts
    PERFORM pg_advisory_xact_lock(12345); -- Use advisory lock to prevent concurrent refreshes
    
    RAISE NOTICE 'Auto-refreshing materialized views due to changes in %', TG_TABLE_NAME;
    
    -- Refresh materialized views
    PERFORM refresh_charging_station_views();
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Drop existing triggers if any
DROP TRIGGER IF EXISTS trg_refresh_mv_after_stations ON stations;
DROP TRIGGER IF EXISTS trg_refresh_mv_after_connectors ON connectors;
DROP TRIGGER IF EXISTS trg_refresh_mv_after_connector_types ON connector_types;

-- ==========================================
-- Create statement-level triggers for automatic refresh
CREATE TRIGGER trg_refresh_mv_after_stations
    AFTER INSERT OR UPDATE OR DELETE ON stations
    FOR EACH STATEMENT
    EXECUTE FUNCTION trg_refresh_mv_immediate();

CREATE TRIGGER trg_refresh_mv_after_connectors
    AFTER INSERT OR UPDATE OR DELETE ON connectors
    FOR EACH STATEMENT
    EXECUTE FUNCTION trg_refresh_mv_immediate();

CREATE TRIGGER trg_refresh_mv_after_connector_types
    AFTER INSERT OR UPDATE OR DELETE ON connector_types
    FOR EACH STATEMENT
    EXECUTE FUNCTION trg_refresh_mv_immediate();

-- ==========================================
-- Utility Functions

-- Function to manually refresh all views
CREATE OR REPLACE FUNCTION refresh_all_materialized_views()
RETURNS TEXT AS $$
BEGIN
    PERFORM refresh_charging_station_views();
    RETURN 'All materialized views refreshed successfully at ' || NOW();
END;
$$ LANGUAGE plpgsql;

-- Function to find stations within radius
CREATE OR REPLACE FUNCTION find_stations_nearby(
    p_longitude FLOAT,
    p_latitude FLOAT, 
    p_radius_km FLOAT DEFAULT 10,
    p_min_power_kw FLOAT DEFAULT 0,
    p_connector_types TEXT[] DEFAULT NULL
) RETURNS TABLE(
    station_id BIGINT,
    name VARCHAR,
    address TEXT,
    distance_km FLOAT,
    max_power_kw DECIMAL,
    available_connectors INTEGER,
    connector_types TEXT[]
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        g.station_id,
        g.name,
        g.address,
        ST_Distance(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326)) / 1000 as distance_km,
        g.max_power_kw,
        g.total_available_connectors as available_connectors,
        g.available_connector_names as connector_types
    FROM mv_charging_stations_geo g
    WHERE 
        ST_DWithin(g.location, ST_SetSRID(ST_MakePoint(p_longitude, p_latitude), 4326), p_radius_km * 1000)
        AND g.max_power_kw >= p_min_power_kw
        AND g.has_available_connectors = true
        AND (p_connector_types IS NULL OR g.available_connector_names && p_connector_types)
    ORDER BY distance_km;
END;
$$ LANGUAGE plpgsql;

-- Function to get station statistics
CREATE OR REPLACE FUNCTION get_station_statistics()
RETURNS TABLE(
    total_stations BIGINT,
    operational_stations BIGINT,
    total_connectors BIGINT,
    available_connectors BIGINT,
    avg_power_kw DECIMAL,
    max_power_kw DECIMAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(DISTINCT s.station_id) as total_stations,
        COUNT(DISTINCT CASE WHEN s.is_operational = true THEN s.station_id END) as operational_stations,
        COUNT(c.connector_id) as total_connectors,
        COUNT(CASE WHEN c.status = 'available' THEN 1 END) as available_connectors,
        AVG(c.power_level_kw) as avg_power_kw,
        MAX(c.power_level_kw) as max_power_kw
    FROM stations s
    LEFT JOIN connectors c ON s.station_id = c.station_id
    WHERE s.status = 'verified';
END;
$$ LANGUAGE plpgsql;

-- Function to check trigger status
CREATE OR REPLACE FUNCTION check_mv_refresh_status()
RETURNS TABLE(
    trigger_name TEXT,
    table_name TEXT,
    is_enabled BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        tgname::TEXT as trigger_name,
        relname::TEXT as table_name,
        tgenabled = 'O' as is_enabled
    FROM pg_trigger t
    JOIN pg_class c ON t.tgrelid = c.oid
    WHERE tgname LIKE 'trg_refresh_mv%'
    ORDER BY relname;
END;
$$ LANGUAGE plpgsql;

-- Function to temporarily disable auto-refresh
CREATE OR REPLACE FUNCTION disable_auto_refresh()
RETURNS TEXT AS $$
BEGIN
    ALTER TABLE stations DISABLE TRIGGER trg_refresh_mv_after_stations;
    ALTER TABLE connectors DISABLE TRIGGER trg_refresh_mv_after_connectors;
    ALTER TABLE connector_types DISABLE TRIGGER trg_refresh_mv_after_connector_types;
    RETURN 'Auto-refresh triggers disabled';
END;
$$ LANGUAGE plpgsql;

-- Function to re-enable auto-refresh
CREATE OR REPLACE FUNCTION enable_auto_refresh()
RETURNS TEXT AS $$
BEGIN
    ALTER TABLE stations ENABLE TRIGGER trg_refresh_mv_after_stations;
    ALTER TABLE connectors ENABLE TRIGGER trg_refresh_mv_after_connectors;
    ALTER TABLE connector_types ENABLE TRIGGER trg_refresh_mv_after_connector_types;
    RETURN 'Auto-refresh triggers enabled';
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Display trigger status
DO $$
BEGIN
    RAISE NOTICE 'Automatic materialized view refresh triggers installed:';
    RAISE NOTICE '- Stations table: trg_refresh_mv_after_stations';
    RAISE NOTICE '- Connectors table: trg_refresh_mv_after_connectors'; 
    RAISE NOTICE '- Connector Types table: trg_refresh_mv_after_connector_types';
    RAISE NOTICE '';
    RAISE NOTICE 'Materialized views will now refresh automatically after data changes.';
    RAISE NOTICE 'Use check_mv_refresh_status() to verify trigger status.';
    RAISE NOTICE 'Use disable_auto_refresh() / enable_auto_refresh() for bulk operations.';
END $$;