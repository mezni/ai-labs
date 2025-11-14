-- ==========================================
-- Enhanced OSM staging table with better indexing
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
    imported_at TIMESTAMPTZ DEFAULT NOW(),
    processed BOOLEAN DEFAULT FALSE
);

-- Enhanced indexes for better performance
CREATE INDEX IF NOT EXISTS idx_osm_temp_geom ON osm_charging_stations_temp USING GIST(geom);
CREATE INDEX IF NOT EXISTS idx_osm_temp_osm_id ON osm_charging_stations_temp(osm_id);
CREATE INDEX IF NOT EXISTS idx_osm_temp_processed ON osm_charging_stations_temp(processed);
CREATE INDEX IF NOT EXISTS idx_osm_temp_operator ON osm_charging_stations_temp(operator);

-- ==========================================
-- Enhanced connector extraction with bulk operations
CREATE OR REPLACE FUNCTION extract_connectors_from_osm_tags(
    p_station_id BIGINT,
    p_tags HSTORE,
    p_created_by TEXT
) RETURNS INTEGER AS $$
DECLARE
    v_connector_count INTEGER := 0;
    v_connector_types TEXT[] := ARRAY['type2', 'ccs', 'chademo', 'tesla', 'type1', 'schuko'];
    v_connector_type TEXT;
    v_count INTEGER;
    v_power DECIMAL(5,2);
    v_type_id INTEGER;
    v_connector_data connectors[];
BEGIN
    -- Delete existing connectors for this station
    DELETE FROM connectors WHERE station_id = p_station_id;
    
    -- Process each connector type
    FOREACH v_connector_type IN ARRAY v_connector_types LOOP
        -- Check if this connector type exists in tags
        IF p_tags ? ('socket:' || v_connector_type) THEN
            BEGIN
                v_count := (p_tags->('socket:' || v_connector_type))::INTEGER;
                IF v_count IS NULL OR v_count <= 0 THEN
                    CONTINUE;
                END IF;
                
                -- Determine power output with sensible defaults
                v_power := COALESCE(
                    NULLIF(p_tags->('socket:' || v_connector_type || ':output'), '')::DECIMAL,
                    CASE v_connector_type
                        WHEN 'type2' THEN 22.0
                        WHEN 'ccs' THEN 50.0
                        WHEN 'chademo' THEN 50.0
                        WHEN 'tesla' THEN 120.0
                        WHEN 'type1' THEN 7.4
                        WHEN 'schuko' THEN 3.7
                        ELSE 22.0
                    END
                );
                
                -- Get connector type ID
                SELECT connector_type_id INTO v_type_id 
                FROM connector_types 
                WHERE name = v_connector_type;
                
                IF v_type_id IS NULL THEN
                    RAISE NOTICE 'Connector type % not found, creating placeholder', v_connector_type;
                    -- Create placeholder connector type if missing
                    INSERT INTO connector_types (name, description, current_type, typical_power_kw, created_by)
                    VALUES (
                        v_connector_type,
                        'Auto-created from OSM import',
                        CASE 
                            WHEN v_connector_type IN ('ccs', 'chademo', 'tesla') THEN 'DC'
                            ELSE 'AC'
                        END,
                        v_power,
                        p_created_by
                    )
                    RETURNING connector_type_id INTO v_type_id;
                END IF;
                
                -- Insert all connectors of this type at once
                INSERT INTO connectors (
                    station_id, connector_type_id, power_level_kw, status, 
                    created_by, created_at
                )
                SELECT 
                    p_station_id, 
                    v_type_id, 
                    v_power, 
                    'available',
                    p_created_by,
                    NOW()
                FROM generate_series(1, v_count);
                
                v_connector_count := v_connector_count + v_count;
                
            EXCEPTION WHEN OTHERS THEN
                RAISE NOTICE 'Error processing connector type %: %', v_connector_type, SQLERRM;
            END;
        END IF;
    END LOOP;
    
    RETURN v_connector_count;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Enhanced OSM sync function with bulk operations
CREATE OR REPLACE FUNCTION sync_osm_charging_stations(
    p_created_by TEXT DEFAULT 'osm_import',
    p_batch_size INTEGER DEFAULT 1000
) RETURNS TABLE(
    stations_processed INTEGER,
    stations_created INTEGER,
    stations_updated INTEGER,
    connectors_created INTEGER,
    errors INTEGER
) AS $$
DECLARE
    v_total_processed INTEGER := 0;
    v_total_created INTEGER := 0;
    v_total_updated INTEGER := 0;
    v_total_connectors INTEGER := 0;
    v_total_errors INTEGER := 0;
    v_batch_counter INTEGER := 0;
    r RECORD;
BEGIN
    RAISE NOTICE 'Starting OSM sync for % stations (batch size: %)', 
        (SELECT COUNT(*) FROM osm_charging_stations_temp WHERE NOT processed), 
        p_batch_size;
    
    -- Process stations in batches for better performance
    FOR r IN 
        SELECT * 
        FROM osm_charging_stations_temp 
        WHERE NOT processed OR processed IS NULL
        ORDER BY osm_id
        LIMIT p_batch_size
    LOOP
        BEGIN
            -- Disable auto-refresh during batch processing
            PERFORM disable_auto_refresh();
            
            DECLARE
                v_station_id BIGINT;
                v_network_id INTEGER;
                v_action TEXT;
                v_connectors_added INTEGER;
                v_station_tags HSTORE;
            BEGIN
                -- Enhanced network creation with better matching
                v_network_id := NULL;
                IF r.operator IS NOT NULL AND r.operator != '' THEN
                    -- Try exact match first, then case-insensitive
                    SELECT network_id INTO v_network_id 
                    FROM networks 
                    WHERE name ILIKE r.operator 
                    LIMIT 1;
                    
                    IF v_network_id IS NULL THEN
                        INSERT INTO networks(
                            name, type, contact_email, created_by, created_at
                        ) VALUES(
                            r.operator, 'company', NULL, p_created_by, NOW()
                        )
                        RETURNING network_id INTO v_network_id;
                        RAISE NOTICE 'Created new network: % (ID: %)', r.operator, v_network_id;
                    END IF;
                END IF;

                -- Build comprehensive tags
                v_station_tags := COALESCE(r.tags, hstore('')) || hstore(array[
                    ['osm_imported', 'true'],
                    ['osm_operator', r.operator],
                    ['opening_hours', r.opening_hours],
                    ['capacity', r.capacity::text],
                    ['fee', r.fee],
                    ['parking_fee', r.parking_fee],
                    ['access', r.access],
                    ['socket_type2_count', r.socket_type2::text],
                    ['socket_ccs_count', r.socket_ccs::text],
                    ['socket_chademo_count', r.socket_chademo::text],
                    ['socket_type2_output', r.socket_type2_output::text],
                    ['socket_ccs_output', r.socket_ccs_output::text],
                    ['socket_chademo_output', r.socket_chademo_output::text]
                ]);

                -- Insert/update station with conflict resolution
                INSERT INTO stations(
                    network_id, name, address, city, state, country, postal_code, 
                    location, tags, osm_id, status, is_operational, created_by, created_at
                ) VALUES (
                    v_network_id, 
                    COALESCE(r.name, 'Unnamed Station'),
                    COALESCE(r.address, ''),
                    COALESCE(r.city, ''),
                    COALESCE(r.state, ''),
                    COALESCE(r.country, ''),
                    COALESCE(r.postal_code, ''),
                    ST_SetSRID(ST_MakePoint(r.longitude, r.latitude), 4326),
                    v_station_tags,
                    r.osm_id, 
                    'verified',  -- Auto-verify OSM imports
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
                    tags = stations.tags || EXCLUDED.tags,  -- Merge tags instead of replace
                    network_id = COALESCE(EXCLUDED.network_id, stations.network_id),
                    status = 'verified',  -- Re-verify on update
                    updated_by = p_created_by,
                    updated_at = NOW()
                RETURNING station_id INTO v_station_id;

                -- Determine action and count
                IF FOUND THEN
                    IF (SELECT created_at FROM stations WHERE station_id = v_station_id) = NOW() THEN
                        v_action := 'created';
                        v_total_created := v_total_created + 1;
                    ELSE
                        v_action := 'updated';
                        v_total_updated := v_total_updated + 1;
                    END IF;
                ELSE
                    SELECT station_id INTO v_station_id FROM stations WHERE osm_id = r.osm_id;
                    v_action := 'skipped';
                END IF;

                -- Process connectors
                v_connectors_added := extract_connectors_from_osm_tags(
                    v_station_id, 
                    v_station_tags, 
                    p_created_by
                );
                v_total_connectors := v_total_connectors + v_connectors_added;

                -- Mark as processed
                UPDATE osm_charging_stations_temp 
                SET processed = TRUE 
                WHERE osm_id = r.osm_id;

                v_total_processed := v_total_processed + 1;
                v_batch_counter := v_batch_counter + 1;

                -- Commit batch periodically
                IF v_batch_counter >= 100 THEN
                    RAISE NOTICE 'Processed % stations...', v_total_processed;
                    v_batch_counter := 0;
                END IF;

            EXCEPTION WHEN OTHERS THEN
                RAISE NOTICE 'Error processing station %: %', r.osm_id, SQLERRM;
                v_total_errors := v_total_errors + 1;
                
                -- Mark as processed even on error to avoid infinite retry
                UPDATE osm_charging_stations_temp 
                SET processed = TRUE 
                WHERE osm_id = r.osm_id;
            END;
            
        END;
    END LOOP;

    -- Re-enable auto-refresh and refresh materialized views
    PERFORM enable_auto_refresh();
    PERFORM refresh_charging_station_views();

    RAISE NOTICE 'OSM sync completed: % processed, % created, % updated, % connectors added, % errors',
        v_total_processed, v_total_created, v_total_updated, v_total_connectors, v_total_errors;

    RETURN QUERY SELECT 
        v_total_processed, 
        v_total_created, 
        v_total_updated, 
        v_total_connectors, 
        v_total_errors;

EXCEPTION WHEN OTHERS THEN
    -- Ensure auto-refresh is re-enabled even on fatal errors
    PERFORM enable_auto_refresh();
    RAISE EXCEPTION 'OSM sync failed: %', SQLERRM;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function to clear processed OSM data
CREATE OR REPLACE FUNCTION clear_processed_osm_data()
RETURNS INTEGER AS $$
DECLARE
    v_deleted_count INTEGER;
BEGIN
    DELETE FROM osm_charging_stations_temp 
    WHERE processed = TRUE;
    
    GET DIAGNOSTICS v_deleted_count = ROW_COUNT;
    RAISE NOTICE 'Cleared % processed OSM records', v_deleted_count;
    
    RETURN v_deleted_count;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function to get OSM sync statistics
CREATE OR REPLACE FUNCTION get_osm_sync_stats()
RETURNS TABLE(
    total_records BIGINT,
    processed_records BIGINT,
    pending_records BIGINT,
    last_import TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(*) as total_records,
        COUNT(*) FILTER (WHERE processed = TRUE) as processed_records,
        COUNT(*) FILTER (WHERE processed = FALSE OR processed IS NULL) as pending_records,
        MAX(imported_at) as last_import
    FROM osm_charging_stations_temp;
END;
$$ LANGUAGE plpgsql;

-- ==========================================
-- Function to reset OSM processing status
CREATE OR REPLACE FUNCTION reset_osm_processing_status()
RETURNS INTEGER AS $$
DECLARE
    v_reset_count INTEGER;
BEGIN
    UPDATE osm_charging_stations_temp 
    SET processed = FALSE 
    WHERE processed = TRUE;
    
    GET DIAGNOSTICS v_reset_count = ROW_COUNT;
    RAISE NOTICE 'Reset processing status for % OSM records', v_reset_count;
    
    RETURN v_reset_count;
END;
$$ LANGUAGE plpgsql;