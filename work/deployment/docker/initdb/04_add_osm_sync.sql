-- ==========================================
-- OSM staging table
CREATE TABLE IF NOT EXISTS osm_charging_stations_temp (
    osm_id BIGINT PRIMARY KEY,
    name VARCHAR(255),
    address TEXT,
    longitude FLOAT,
    latitude FLOAT,
    operator VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100),
    postal_code VARCHAR(20),
    opening_hours TEXT,
    capacity INTEGER,
    fee TEXT,
    parking_fee TEXT,
    access TEXT,
    socket_type2 INTEGER,
    socket_ccs INTEGER,
    socket_chademo INTEGER,
    socket_type2_output DECIMAL(5,2),
    socket_ccs_output DECIMAL(5,2),
    socket_chademo_output DECIMAL(5,2),
    tags HSTORE,
    geom GEOMETRY(Point,4326),
    imported_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_osm_temp_geom ON osm_charging_stations_temp USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_osm_temp_osm_id ON osm_charging_stations_temp(osm_id);

-- ==========================================
-- Extract connectors from OSM tags
CREATE OR REPLACE FUNCTION extract_connectors_from_osm_tags(
    p_station_id BIGINT,
    p_tags HSTORE,
    p_created_by TEXT
) RETURNS VOID AS $$
DECLARE
    v_count INTEGER;
    v_power DECIMAL(5,2);
    v_type_id INTEGER;
    i INTEGER;
    v_connector_types TEXT[] := ARRAY['type2', 'ccs', 'chademo'];
    v_connector_type TEXT;
    v_deleted_count INTEGER;
    v_added_count INTEGER := 0;
BEGIN
    RAISE NOTICE 'Extracting connectors for station %, tags: %', p_station_id, p_tags;
    
    -- Delete existing connectors for this station
    DELETE FROM connectors WHERE station_id = p_station_id;
    GET DIAGNOSTICS v_deleted_count = ROW_COUNT;
    RAISE NOTICE 'Deleted % existing connectors', v_deleted_count;
    
    -- Process each connector type
    FOREACH v_connector_type IN ARRAY v_connector_types LOOP
        -- Check if this connector type exists in tags
        IF p_tags ? ('socket:' || v_connector_type) THEN
            BEGIN
                v_count := (p_tags->('socket:' || v_connector_type))::INTEGER;
                v_power := COALESCE(
                    NULLIF(p_tags->('socket:' || v_connector_type || ':output'), '')::DECIMAL,
                    CASE v_connector_type
                        WHEN 'type2' THEN 22.0
                        WHEN 'ccs' THEN 50.0
                        WHEN 'chademo' THEN 50.0
                        ELSE 22.0
                    END
                );
                
                -- Get connector type ID
                SELECT connector_type_id INTO v_type_id 
                FROM connector_types 
                WHERE name = v_connector_type;
                
                IF v_type_id IS NULL THEN
                    RAISE NOTICE 'Connector type % not found, skipping', v_connector_type;
                    CONTINUE;
                END IF;
                
                RAISE NOTICE 'Adding % connectors of type % with power % kW', v_count, v_connector_type, v_power;
                
                -- Insert connectors
                FOR i IN 1..v_count LOOP
                    INSERT INTO connectors(
                        station_id, connector_type_id, power_level_kw, status, 
                        created_by, created_at
                    ) VALUES (
                        p_station_id, v_type_id, v_power, 'available', 
                        p_created_by, NOW()
                    );
                    v_added_count := v_added_count + 1;
                END LOOP;
                
            EXCEPTION WHEN OTHERS THEN
                RAISE NOTICE 'Error processing connector type %: %', v_connector_type, SQLERRM;
            END;
        END IF;
    END LOOP;
    
    RAISE NOTICE 'Added % new connectors for station %', v_added_count, p_station_id;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Full OSM sync function (FIXED VERSION)
CREATE OR REPLACE FUNCTION sync_osm_charging_stations(p_created_by TEXT DEFAULT 'osm_import')
RETURNS TABLE(station_id BIGINT, osm_id BIGINT, action TEXT) AS $$
DECLARE
    r RECORD;
    v_station_id BIGINT;
    v_network_id INTEGER;
    v_action TEXT;
    v_processed INTEGER := 0;
    v_errors INTEGER := 0;
BEGIN
    RAISE NOTICE 'Starting OSM sync for % stations', (SELECT COUNT(*) FROM osm_charging_stations_temp);
    
    FOR r IN SELECT * FROM osm_charging_stations_temp LOOP
        BEGIN
            -- Network creation if missing
            v_network_id := NULL;
            IF r.operator IS NOT NULL AND r.operator != '' THEN
                SELECT network_id INTO v_network_id FROM networks WHERE name = r.operator LIMIT 1;
                IF v_network_id IS NULL THEN
                    INSERT INTO networks(name, type, contact_email, created_by, created_at)
                    VALUES(r.operator, 'company', NULL, p_created_by, NOW())
                    RETURNING network_id INTO v_network_id;
                    RAISE NOTICE 'Created new network: % (ID: %)', r.operator, v_network_id;
                END IF;
            END IF;

            -- Insert/update station
            INSERT INTO stations(
                network_id, name, address, city, state, country, postal_code, 
                location, tags, osm_id, status, is_operational, created_by, created_at
            ) VALUES (
                v_network_id, 
                COALESCE(r.name, 'Unnamed Station'),
                r.address, r.city, r.state, r.country, r.postal_code,
                ST_SetSRID(ST_MakePoint(r.longitude, r.latitude), 4326),
                COALESCE(r.tags, hstore('')),
                r.osm_id, 
                'onboarding', 
                TRUE, 
                p_created_by, 
                NOW()
            )
            ON CONFLICT (osm_id) DO UPDATE SET
                name = EXCLUDED.name,
                address = EXCLUDED.address,
                city = EXCLUDED.city,
                state = EXCLUDED.state,
                country = EXCLUDED.country,
                postal_code = EXCLUDED.postal_code,
                location = EXCLUDED.location,
                tags = EXCLUDED.tags,
                network_id = COALESCE(EXCLUDED.network_id, stations.network_id),
                updated_by = p_created_by,
                updated_at = NOW()
            RETURNING station_id INTO v_station_id;

            -- Determine action type
            IF FOUND THEN
                v_action := CASE 
                    WHEN (SELECT created_at FROM stations WHERE station_id = v_station_id) = NOW() 
                    THEN 'inserted' 
                    ELSE 'updated' 
                END;
            ELSE
                SELECT station_id INTO v_station_id FROM stations WHERE osm_id = r.osm_id;
                v_action := 'skipped';
            END IF;

            -- Insert connectors
            PERFORM extract_connectors_from_osm_tags(v_station_id, COALESCE(r.tags, hstore('')), p_created_by);
            
            v_processed := v_processed + 1;
            
            -- FIX: Assign values to output parameters and return
            station_id := v_station_id;
            osm_id := r.osm_id;
            action := v_action;
            RETURN NEXT;
            
        EXCEPTION WHEN OTHERS THEN
            v_errors := v_errors + 1;
            RAISE NOTICE 'Error processing OSM ID %: %', r.osm_id, SQLERRM;
            
            -- Return error information
            station_id := NULL;
            osm_id := r.osm_id;
            action := 'error: ' || SQLERRM;
            RETURN NEXT;
        END;
    END LOOP;

    -- Refresh materialized views
    RAISE NOTICE 'Processed: %, Errors: %, Refreshing materialized views...', v_processed, v_errors;
    PERFORM refresh_charging_station_views();
    
    RAISE NOTICE 'OSM sync completed successfully';
END;
$$ LANGUAGE plpgsql;


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