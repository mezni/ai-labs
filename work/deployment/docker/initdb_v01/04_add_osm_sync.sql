-- Create OSM staging table
CREATE TABLE IF NOT EXISTS osm_charging_stations_temp (
    osm_id BIGINT PRIMARY KEY,
    name VARCHAR(255),
    address TEXT,
    longitude FLOAT,
    latitude FLOAT,
    network VARCHAR(255),  
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
    geom GEOMETRY(Point, 4326),
    imported_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_osm_temp_geom ON osm_charging_stations_temp USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_osm_temp_osm_id ON osm_charging_stations_temp (osm_id);

-- Function to extract connectors from OSM tags
CREATE OR REPLACE FUNCTION extract_connectors_from_osm_tags(
    p_station_id INTEGER, 
    p_tags HSTORE, 
    p_user_id TEXT
) RETURNS INTEGER AS $$
DECLARE
    v_connector_count INTEGER := 0;
    v_count_total INTEGER;
    v_power_kw DECIMAL(5,2);
    v_connector_type_id INTEGER;
    v_connector_name TEXT;
BEGIN
    -- Clear existing connectors for this station
    DELETE FROM connectors WHERE station_id = p_station_id;
    
    -- Insert Type2 connectors
    IF p_tags ? 'socket:type2' AND p_tags->'socket:type2' ~ '^\d+$' THEN
        v_count_total := (p_tags->'socket:type2')::INTEGER;
        v_power_kw := NULLIF(p_tags->'socket:type2:output', '')::DECIMAL;
        v_connector_name := 'Type 2 (Mennekes)';
        
        -- Get connector type ID
        SELECT connector_type_id INTO v_connector_type_id 
        FROM connector_types 
        WHERE name = v_connector_name;
        
        IF v_connector_type_id IS NOT NULL THEN
            INSERT INTO connectors (
                station_id, connector_type_id, power_level_kw, status,
                max_voltage, max_amperage, created_by, created_at
            ) VALUES (
                p_station_id,
                v_connector_type_id,
                COALESCE(v_power_kw, 22.0),
                'available',
                400,  -- Typical for Type 2
                32,   -- Typical for Type 2
                p_user_id,
                NOW()
            );
            v_connector_count := v_connector_count + 1;
        END IF;
    END IF;
    
    -- Insert CCS connectors
    IF p_tags ? 'socket:ccs' AND p_tags->'socket:ccs' ~ '^\d+$' THEN
        v_count_total := (p_tags->'socket:ccs')::INTEGER;
        v_power_kw := NULLIF(p_tags->'socket:ccs:output', '')::DECIMAL;
        v_connector_name := 'CCS2 (Combo 2)';
        
        SELECT connector_type_id INTO v_connector_type_id 
        FROM connector_types 
        WHERE name = v_connector_name;
        
        IF v_connector_type_id IS NOT NULL THEN
            INSERT INTO connectors (
                station_id, connector_type_id, power_level_kw, status,
                max_voltage, max_amperage, created_by, created_at
            ) VALUES (
                p_station_id,
                v_connector_type_id,
                COALESCE(v_power_kw, 50.0),
                'available',
                800,  -- Typical for CCS2
                500,  -- Typical for CCS2
                p_user_id,
                NOW()
            );
            v_connector_count := v_connector_count + 1;
        END IF;
    END IF;
    
    -- Insert CHAdeMO connectors
    IF p_tags ? 'socket:chademo' AND p_tags->'socket:chademo' ~ '^\d+$' THEN
        v_count_total := (p_tags->'socket:chademo')::INTEGER;
        v_power_kw := NULLIF(p_tags->'socket:chademo:output', '')::DECIMAL;
        v_connector_name := 'CHAdeMO';
        
        SELECT connector_type_id INTO v_connector_type_id 
        FROM connector_types 
        WHERE name = v_connector_name;
        
        IF v_connector_type_id IS NOT NULL THEN
            INSERT INTO connectors (
                station_id, connector_type_id, power_level_kw, status,
                max_voltage, max_amperage, created_by, created_at
            ) VALUES (
                p_station_id,
                v_connector_type_id,
                COALESCE(v_power_kw, 50.0),
                'available',
                600,  -- Typical for CHAdeMO
                350,  -- Typical for CHAdeMO
                p_user_id,
                NOW()
            );
            v_connector_count := v_connector_count + 1;
        END IF;
    END IF;

    -- Handle Type 1 connectors (common in OSM)
    IF p_tags ? 'socket:type1' AND p_tags->'socket:type1' ~ '^\d+$' THEN
        v_count_total := (p_tags->'socket:type1')::INTEGER;
        v_power_kw := NULLIF(p_tags->'socket:type1:output', '')::DECIMAL;
        v_connector_name := 'Type 1 (J1772)';
        
        SELECT connector_type_id INTO v_connector_type_id 
        FROM connector_types 
        WHERE name = v_connector_name;
        
        IF v_connector_type_id IS NOT NULL THEN
            INSERT INTO connectors (
                station_id, connector_type_id, power_level_kw, status,
                max_voltage, max_amperage, created_by, created_at
            ) VALUES (
                p_station_id,
                v_connector_type_id,
                COALESCE(v_power_kw, 7.2),
                'available',
                240,  -- Typical for Type 1
                32,   -- Typical for Type 1
                p_user_id,
                NOW()
            );
            v_connector_count := v_connector_count + 1;
        END IF;
    END IF;

    RETURN v_connector_count;
END;
$$ LANGUAGE plpgsql;

-- Function to find or create network based on OSM network tag
CREATE OR REPLACE FUNCTION get_or_create_network_from_osm(
    p_network_name TEXT,
    p_user_id TEXT DEFAULT 'system'
) RETURNS INTEGER AS $$
DECLARE
    v_network_id INTEGER;
    v_clean_name TEXT;
BEGIN
    -- Clean the network name
    v_clean_name := TRIM(COALESCE(p_network_name, 'OpenStreetMap Community'));
    
    -- Try to find existing network
    SELECT network_id INTO v_network_id 
    FROM networks 
    WHERE name = v_clean_name;
    
    -- If not found, create a new network
    IF v_network_id IS NULL THEN
        INSERT INTO networks (name, type, contact_email, created_by)
        VALUES (
            v_clean_name,
            'company',
            'info@' || LOWER(REPLACE(v_clean_name, ' ', '')) || '.com',
            p_user_id
        )
        RETURNING network_id INTO v_network_id;
        
        -- Also create a company entry if it's a company type network
        IF v_clean_name != 'OpenStreetMap Community' THEN
            INSERT INTO companies (network_id, created_by)
            VALUES (v_network_id, p_user_id);
        END IF;
    END IF;
    
    RETURN v_network_id;
END;
$$ LANGUAGE plpgsql;

-- Main OSM sync function
CREATE OR REPLACE FUNCTION sync_osm_charging_stations(
    p_default_network_id INTEGER DEFAULT NULL,
    p_user_id TEXT DEFAULT 'system'
) RETURNS TABLE(
    updated_count INTEGER,
    inserted_count INTEGER,
    deactivated_count INTEGER
) AS $$
DECLARE
    v_updated_count INTEGER := 0;
    v_inserted_count INTEGER := 0;
    v_deactivated_count INTEGER := 0;
    v_station_id INTEGER;
    v_tags HSTORE;
    v_network_id INTEGER;
    v_osm_network TEXT;
BEGIN
    -- Update existing stations
    WITH updated AS (
        UPDATE stations s
        SET 
            name = COALESCE(osm.name, s.name),
            address = COALESCE(osm.address, s.address),
            city = COALESCE(NULLIF(osm.tags->'addr:city', ''), s.city),
            country = COALESCE(NULLIF(osm.tags->'addr:country', ''), s.country),
            tags = s.tags || hstore(array[
                ['network', osm.network],  -- Changed from operator to network
                ['opening_hours', osm.opening_hours],
                ['capacity', osm.capacity::text],
                ['fee', osm.fee],
                ['parking_fee', osm.parking_fee],
                ['access', osm.access],
                ['socket:type2', osm.socket_type2::text],
                ['socket:ccs', osm.socket_ccs::text],
                ['socket:chademo', osm.socket_chademo::text],
                ['socket:type2:output', osm.socket_type2_output::text],
                ['socket:ccs:output', osm.socket_ccs_output::text],
                ['socket:chademo:output', osm.socket_chademo_output::text]
            ]) || osm.tags,
            updated_by = p_user_id,
            updated_at = NOW()
        FROM osm_charging_stations_temp osm
        WHERE s.osm_id = osm.osm_id
        AND (
            s.name IS DISTINCT FROM osm.name OR
            s.address IS DISTINCT FROM osm.address OR
            s.tags IS DISTINCT FROM (
                hstore(array[
                    ['network', osm.network],  -- Changed from operator to network
                    ['opening_hours', osm.opening_hours],
                    ['capacity', osm.capacity::text],
                    ['fee', osm.fee],
                    ['parking_fee', osm.parking_fee],
                    ['access', osm.access],
                    ['socket:type2', osm.socket_type2::text],
                    ['socket:ccs', osm.socket_ccs::text],
                    ['socket:chademo', osm.socket_chademo::text],
                    ['socket:type2:output', osm.socket_type2_output::text],
                    ['socket:ccs:output', osm.socket_ccs_output::text],
                    ['socket:chademo:output', osm.socket_chademo_output::text]
                ]) || osm.tags
            )
        )
        RETURNING s.station_id, s.tags
    )
    SELECT COUNT(*) INTO v_updated_count FROM updated;

    -- Process connectors for updated stations
    FOR v_station_id, v_tags IN 
        SELECT s.station_id, s.tags 
        FROM stations s
        INNER JOIN osm_charging_stations_temp osm ON s.osm_id = osm.osm_id
        WHERE s.updated_at >= NOW() - INTERVAL '5 minutes'
    LOOP
        PERFORM extract_connectors_from_osm_tags(v_station_id, v_tags, p_user_id);
    END LOOP;

    -- Insert new stations
    WITH inserted AS (
        INSERT INTO stations (
            network_id, osm_id, name, address, city, country, 
            location, tags, status, is_operational, created_by, created_at
        )
        SELECT 
            COALESCE(
                get_or_create_network_from_osm(osm.network, p_user_id),
                p_default_network_id,
                get_or_create_network_from_osm('OpenStreetMap Community', p_user_id)
            ),
            osm.osm_id,
            osm.name,
            osm.address,
            NULLIF(osm.tags->'addr:city', ''),
            NULLIF(osm.tags->'addr:country', ''),
            osm.geom::GEOGRAPHY,
            hstore(array[
                ['network', osm.network],  -- Changed from operator to network
                ['opening_hours', osm.opening_hours],
                ['capacity', osm.capacity::text],
                ['fee', osm.fee],
                ['parking_fee', osm.parking_fee],
                ['access', osm.access],
                ['socket:type2', osm.socket_type2::text],
                ['socket:ccs', osm.socket_ccs::text],
                ['socket:chademo', osm.socket_chademo::text],
                ['socket:type2:output', osm.socket_type2_output::text],
                ['socket:ccs:output', osm.socket_ccs_output::text],
                ['socket:chademo:output', osm.socket_chademo_output::text]
            ]) || osm.tags,
            'verified',  -- Auto-verify OSM stations
            TRUE,        -- Assume operational
            p_user_id,
            NOW()
        FROM osm_charging_stations_temp osm
        WHERE NOT EXISTS (
            SELECT 1 FROM stations WHERE osm_id = osm.osm_id
        )
        RETURNING station_id, tags
    )
    SELECT COUNT(*) INTO v_inserted_count FROM inserted;

    -- Process connectors for new stations
    FOR v_station_id, v_tags IN 
        SELECT s.station_id, s.tags 
        FROM stations s
        INNER JOIN osm_charging_stations_temp osm ON s.osm_id = osm.osm_id
        WHERE s.created_at >= NOW() - INTERVAL '5 minutes'
    LOOP
        PERFORM extract_connectors_from_osm_tags(v_station_id, v_tags, p_user_id);
    END LOOP;

    -- Refresh materialized views if they exist
    BEGIN
        REFRESH MATERIALIZED VIEW CONCURRENTLY stations_detailed_materialized;
    EXCEPTION WHEN OTHERS THEN
        -- Materialized view might not exist, continue
        NULL;
    END;

    RETURN QUERY SELECT v_updated_count, v_inserted_count, v_deactivated_count;

EXCEPTION WHEN OTHERS THEN
    RAISE EXCEPTION 'OSM sync failed: %', SQLERRM;
END;
$$ LANGUAGE plpgsql;

-- Function to populate OSM temp table from external data
CREATE OR REPLACE FUNCTION populate_osm_temp_from_json(
    p_osm_data JSONB
) RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER := 0;
    v_record JSONB;
BEGIN
    -- Clear existing temp data
    TRUNCATE osm_charging_stations_temp;
    
    -- Insert from JSON array
    FOR v_record IN SELECT * FROM jsonb_array_elements(p_osm_data)
    LOOP
        INSERT INTO osm_charging_stations_temp (
            osm_id, name, address, longitude, latitude,
            network, opening_hours, capacity, fee, parking_fee, access,  -- Changed operator to network
            socket_type2, socket_ccs, socket_chademo,
            socket_type2_output, socket_ccs_output, socket_chademo_output,
            tags, geom
        ) VALUES (
            (v_record->>'id')::BIGINT,
            v_record->>'name',
            COALESCE(v_record->>'addr:full', v_record->>'address'),
            (v_record->'lon')::FLOAT,
            (v_record->'lat')::FLOAT,
            v_record->>'network',  -- Changed from operator to network
            v_record->>'opening_hours',
            NULLIF(v_record->>'capacity', '')::INTEGER,
            v_record->>'fee',
            v_record->>'parking_fee',
            v_record->>'access',
            NULLIF(v_record->>'socket:type2', '')::INTEGER,
            NULLIF(v_record->>'socket:ccs', '')::INTEGER,
            NULLIF(v_record->>'socket:chademo', '')::INTEGER,
            NULLIF(v_record->>'socket:type2:output', '')::DECIMAL,
            NULLIF(v_record->>'socket:ccs:output', '')::DECIMAL,
            NULLIF(v_record->>'socket:chademo:output', '')::DECIMAL,
            hstore((
                SELECT array_agg(key || '=>' || COALESCE(value, ''))
                FROM jsonb_each_text(v_record->'tags')
            )),
            ST_SetSRID(ST_MakePoint(
                (v_record->'lon')::FLOAT,
                (v_record->'lat')::FLOAT
            ), 4326)
        );
        v_count := v_count + 1;
    END LOOP;
    
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Complete sync procedure
CREATE OR REPLACE PROCEDURE complete_osm_sync(
    p_osm_data JSONB DEFAULT NULL,
    p_user_id TEXT DEFAULT 'system'
) AS $$
DECLARE
    v_default_network_id INTEGER;
    v_temp_count INTEGER;
    v_updated_count INTEGER;
    v_inserted_count INTEGER;
    v_deactivated_count INTEGER;
BEGIN
    -- Get default network (OpenStreetMap Community)
    v_default_network_id := get_or_create_network_from_osm('OpenStreetMap Community', p_user_id);
    
    -- Populate temp table if data provided
    IF p_osm_data IS NOT NULL THEN
        v_temp_count := populate_osm_temp_from_json(p_osm_data);
        RAISE NOTICE 'Loaded % records into OSM temp table', v_temp_count;
    END IF;
    
    -- Perform sync
    SELECT updated_count, inserted_count, deactivated_count 
    INTO v_updated_count, v_inserted_count, v_deactivated_count
    FROM sync_osm_charging_stations(v_default_network_id, p_user_id);
    
    RAISE NOTICE 'Sync completed: % updated, % inserted, % deactivated', 
        v_updated_count, v_inserted_count, v_deactivated_count;
    
    -- Clean up temp data older than 1 day
    DELETE FROM osm_charging_stations_temp 
    WHERE imported_at < NOW() - INTERVAL '1 day';
    
END;
$$ LANGUAGE plpgsql;

-- View to see OSM stations with their networks
CREATE OR REPLACE VIEW osm_stations_with_networks AS
SELECT 
    s.station_id,
    s.name AS station_name,
    s.osm_id,
    n.name AS network_name,
    n.network_id,
    s.address,
    s.city,
    s.country,
    COUNT(c.connector_id) AS connector_count,
    s.tags->'network' AS osm_network_tag
FROM stations s
JOIN networks n ON s.network_id = n.network_id
LEFT JOIN connectors c ON s.station_id = c.station_id
WHERE s.osm_id IS NOT NULL
GROUP BY s.station_id, n.name, n.network_id
ORDER BY n.name, s.name;