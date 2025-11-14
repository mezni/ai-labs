CREATE OR REPLACE VIEW vw_charging_stations AS
SELECT
    -- Station info
    s.station_id,
    s.network_id,
    n.name AS network_name,
    n.type AS network_type,
    n.contact_email,
    n.phone_number,
    n.address AS network_address,
    s.name AS station_name,
    s.address AS station_address,
    s.city,
    s.state,
    s.country,
    s.postal_code,
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    s.tags,
    s.status AS station_status,
    s.is_operational,

    -- Quick availability summary
    EXISTS (
        SELECT 1 
        FROM connectors c2
        WHERE c2.station_id = s.station_id AND c2.status = 'available'
    ) AS has_available_connectors,

    -- Total Connector counts
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id
    ) AS total_connectors,

    -- Available Connector counts
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS total_available_connectors,

    -- Total power capacity (all connectors)
    (
        SELECT COALESCE(SUM(c.power_level_kw),0)
        FROM connectors c
        WHERE c.station_id = s.station_id
    ) AS total_power_capacity_kw,

    -- Actual/available power capacity (only available connectors)
    (
        SELECT COALESCE(SUM(c.power_level_kw),0)
        FROM connectors c
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_power_capacity_kw,

    -- Available connector types array
    (
        SELECT ARRAY_AGG(DISTINCT ct.name)
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_connector_names,    

    -- JSON array of connectors
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
                'model', c.model,
                'installation_date', c.installation_date,
                'last_maintenance_date', c.last_maintenance_date
            ) ORDER BY c.power_level_kw DESC NULLS LAST
        )
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id
    ) AS connectors,

    -- Station metadata from tags
    s.tags->'network' AS network,
    s.tags->'opening_hours' AS opening_hours,
    s.tags->'capacity' AS capacity,
    s.tags->'fee' AS fee,
    s.tags->'parking_fee' AS parking_fee,
    s.tags->'access' AS access

FROM stations s
JOIN networks n ON s.network_id = n.network_id;



-- MVIESS
CREATE MATERIALIZED VIEW mv_charging_stations AS
SELECT
    -- Station info
    s.station_id,
    s.network_id,
    n.name AS network_name,
    n.type AS network_type,
    n.contact_email,
    n.phone_number,
    n.address AS network_address,
    s.name AS station_name,
    s.address AS station_address,
    s.city,
    s.state,
    s.country,
    s.postal_code,
    ST_X(s.location::geometry) AS longitude,
    ST_Y(s.location::geometry) AS latitude,
    s.tags,
    s.status AS station_status,
    s.is_operational,

    -- Quick availability summary
    EXISTS (
        SELECT 1 
        FROM connectors c2
        WHERE c2.station_id = s.station_id AND c2.status = 'available'
    ) AS has_available_connectors,

    -- Total Connector counts
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id
    ) AS total_connectors,

    -- Available Connector counts
    (
        SELECT COUNT(*) 
        FROM connectors c 
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS total_available_connectors,

    -- Total power capacity (all connectors)
    (
        SELECT COALESCE(SUM(c.power_level_kw),0)
        FROM connectors c
        WHERE c.station_id = s.station_id
    ) AS total_power_capacity_kw,

    -- Actual/available power capacity (only available connectors)
    (
        SELECT COALESCE(SUM(c.power_level_kw),0)
        FROM connectors c
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_power_capacity_kw,

    -- Available connector types array
    (
        SELECT ARRAY_AGG(DISTINCT ct.name)
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id AND c.status = 'available'
    ) AS available_connector_names,    

    -- JSON array of connectors
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
                'model', c.model,
                'installation_date', c.installation_date,
                'last_maintenance_date', c.last_maintenance_date
            ) ORDER BY c.power_level_kw DESC NULLS LAST
        )
        FROM connectors c
        LEFT JOIN connector_types ct ON c.connector_type_id = ct.connector_type_id
        WHERE c.station_id = s.station_id
    ) AS connectors,

    -- Station metadata from tags
    s.tags->'network' AS network,
    s.tags->'opening_hours' AS opening_hours,
    s.tags->'capacity' AS capacity,
    s.tags->'fee' AS fee,
    s.tags->'parking_fee' AS parking_fee,
    s.tags->'access' AS access

FROM stations s
JOIN networks n ON s.network_id = n.network_id;


CREATE UNIQUE INDEX mv_charging_stations_idx ON mv_charging_stations(station_id);



CREATE OR REPLACE FUNCTION refresh_mv_charging_stations()
RETURNS void AS $$
BEGIN
    -- Try a concurrent refresh
    BEGIN
        REFRESH MATERIALIZED VIEW CONCURRENTLY mv_charging_stations;
    EXCEPTION
        WHEN object_not_in_prerequisite_state THEN
            -- Fallback: non-concurrent refresh (only if index missing or locked)
            REFRESH MATERIALIZED VIEW mv_charging_stations;
    END;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION trg_refresh_mv_stations_connectors()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('refresh_mv_stations', 'run');
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Stations changes
CREATE TRIGGER stations_refresh_mv
AFTER INSERT OR UPDATE OR DELETE
ON stations
FOR EACH STATEMENT
EXECUTE FUNCTION trg_refresh_mv_stations_connectors();

-- Connectors changes
CREATE TRIGGER connectors_refresh_mv
AFTER INSERT OR UPDATE OR DELETE
ON connectors
FOR EACH STATEMENT
EXECUTE FUNCTION trg_refresh_mv_stations_connectors();

CREATE OR REPLACE FUNCTION refresh_mv_listener()
RETURNS void AS $$
DECLARE
    notify_record json;
BEGIN
    -- Start listening
    PERFORM pg_listen('refresh_mv_stations');

    LOOP
        -- Wait for notifications (0.5 sec timeout)
        notify_record := pg_notify_queue(0.5);

        IF notify_record IS NOT NULL THEN
            -- When notified, refresh the MV
            PERFORM refresh_mv_charging_stations();
        END IF;

    END LOOP;

END;
$$ LANGUAGE plpgsql;









tokio::spawn(async move {
    loop {
        sqlx::query("SELECT refresh_mv_listener();")
            .execute(&pool)
            .await
            .ok();
    }
});



SELECT refresh_mv_listener();

