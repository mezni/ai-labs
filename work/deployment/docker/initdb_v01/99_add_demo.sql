
-- ============================
-- 1. Insert Networks
-- ============================
INSERT INTO networks (
    name,
    type,
    contact_email,
    phone_number,
    address,
    created_by,
    updated_by,
    created_at,
    updated_at
) VALUES
('STEG', 'company', 'contact@steg.com.tn', '+216-71-000-001', 'Lac de Tunis, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('Golden Tulip', 'company', 'contact@goldentulip.com.tn', '+216-71-000-002', 'Avenue Ouled Haffouz, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('Tunisia Mall', 'company', 'contact@tunisiamall.com.tn', '+216-71-000-003', 'Les Berges du Lac, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('Energym', 'company', 'contact@energym.com.tn', '+216-71-000-004', 'La Goulette, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('The Residence', 'company', 'contact@theresidence.com.tn', '+216-71-000-005', 'Gammarth, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('Carrefour', 'company', 'contact@carrefour.com.tn', '+216-71-000-006', 'Marsa, Tunis, Tunisia', 'system', 'system', NOW(), NOW()),
('Tunis Air', 'company', 'contact@tunisair.com.tn', '+216-71-000-007', 'Aéroport International de Tunis-Carthage', 'system', 'system', NOW(), NOW()),
('ENNOUR', 'company', 'contact@ennour.com.tn', '+216-71-000-008', 'Route de La Marsa, Carthage, Tunisia', 'system', 'system', NOW(), NOW())
;


-- ============================
-- 2. Insert Stations
-- ============================
INSERT INTO stations (
    network_id,
    osm_id,
    name,
    address,
    city,
    state,
    country,
    postal_code,
    location,
    tags,
    is_operational,
    created_by,
    updated_by,
    created_at,
    updated_at
) VALUES
-- STEG Charging Station - Lac
(1, 202500000001, 'STEG Charging Station - Lac', 'Lac de Tunis, near Tunis City Center', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.2417 36.8380)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','4'],['fee','no'],['parking_fee','no'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Hotel Golden Tulip El Mechtel
(2, 202500000002, 'Hotel Golden Tulip El Mechtel', 'Avenue Ouled Haffouz, Tunis', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.2087 36.8374)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','2'],['fee','yes'],['parking_fee','yes'],['access','customers']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Tunisia Mall Charging Point
(3, 202500000003, 'Tunisia Mall Charging Point', 'Les Berges du Lac, Tunis', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.2376 36.8510)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','6'],['fee','no'],['parking_fee','yes'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Energym Charging Station
(4, 202500000004, 'Energym Charging Station', 'La Goulette, Tunis', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.3050 36.8185)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','8'],['fee','yes'],['parking_fee','no'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- The Residence Tunis
(5, 202500000005, 'The Residence Tunis', 'Gammarth, Tunis', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.3234 36.9542)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','2'],['fee','no'],['parking_fee','no'],['access','customers']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Carrefour Charging Point
(6, 202500000006, 'Carrefour Charging Point', 'Marsa, Tunis', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.3247 36.8782)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','4'],['fee','no'],['parking_fee','no'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Aeroport Tunis-Carthage
(7, 202500000007, 'Aeroport Tunis-Carthage', 'Aéroport International de Tunis-Carthage', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.2272 36.8510)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','4'],['fee','yes'],['parking_fee','yes'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW()),
-- Station ENNOUR
(8, 202500000008, 'Station ENNOUR', 'Route de La Marsa, Carthage', 'Tunis', '', 'Tunisia', '',
 ST_GeogFromText('POINT(10.3215 36.8612)'),
 hstore(ARRAY[['amenity','charging_station'],['capacity','2'],['fee','yes'],['parking_fee','no'],['access','public']]),
 TRUE, 'system', 'system', NOW(), NOW())
;


-- ============================
-- 3. Insert Connectors
-- ============================
INSERT INTO connectors (
    station_id,
    connector_type_id,
    power_level_kw,
    status,
    max_voltage,
    max_amperage,
    serial_number,
    manufacturer,
    model,
    installation_date,
    last_maintenance_date,
    created_by,
    updated_by,
    created_at,
    updated_at
) VALUES
-- STEG Charging Station - Lac (station_id=1)
(1, 2, 22.0, 'available', 400, 32, 'STEG001', 'ABB', 'AC22', '2023-01-10', '2023-10-01', 'system', 'system', NOW(), NOW()),
(1, 4, 350.0, 'available', 800, 500, 'STEG002', 'Siemens', 'DC350', '2023-01-10', '2023-10-01', 'system', 'system', NOW(), NOW()),
-- Hotel Golden Tulip El Mechtel (station_id=2)
(2, 2, 22.0, 'available', 400, 32, 'GT001', 'Schneider', 'AC22', '2023-02-15', '2023-10-05', 'system', 'system', NOW(), NOW()),
(2, 5, 200.0, 'occupied', 600, 350, 'GT002', 'Toshiba', 'CHAdeMO200', '2023-02-15', '2023-10-05', 'system', 'system', NOW(), NOW()),
-- Tunisia Mall Charging Point (station_id=3)
(3, 2, 22.0, 'available', 400, 32, 'TM001', 'ABB', 'AC22', '2023-03-20', '2023-10-10', 'system', 'system', NOW(), NOW()),
(3, 4, 350.0, 'reserved', 800, 500, 'TM002', 'Siemens', 'DC350', '2023-03-20', '2023-10-10', 'system', 'system', NOW(), NOW()),
(3, 5, 200.0, 'available', 600, 350, 'TM003', 'Toshiba', 'CHAdeMO200', '2023-03-20', '2023-10-10', 'system', 'system', NOW(), NOW()),
-- Energym Charging Station (station_id=4)
(4, 2, 22.0, 'out_of_service', 400, 32, 'EN001', 'Schneider', 'AC22', '2023-04-01', '2023-09-30', 'system', 'system', NOW(), NOW()),
(4, 4, 350.0, 'available', 800, 500, 'EN002', 'Siemens', 'DC350', '2023-04-01', '2023-09-30', 'system', 'system', NOW(), NOW()),
-- The Residence Tunis (station_id=5)
(5, 2, 22.0, 'available', 400, 32, 'TR001', 'ABB', 'AC22', '2023-05-05', '2023-10-01', 'system', 'system', NOW(), NOW()),
-- Carrefour Charging Point (station_id=6)
(6, 2, 22.0, 'available', 400, 32, 'CF001', 'Schneider', 'AC22', '2023-06-10', '2023-10-05', 'system', 'system', NOW(), NOW()),
(6, 4, 350.0, 'reserved', 800, 500, 'CF002', 'Siemens', 'DC350', '2023-06-10', '2023-10-05', 'system', 'system', NOW(), NOW()),
-- Aeroport Tunis-Carthage (station_id=7)
(7, 2, 22.0, 'available', 400, 32, 'AT001', 'ABB', 'AC22', '2023-07-15', '2023-10-10', 'system', 'system', NOW(), NOW()),
(7, 4, 350.0, 'available', 800, 500, 'AT002', 'Siemens', 'DC350', '2023-07-15', '2023-10-10', 'system', 'system', NOW(), NOW()),
-- Station ENNOUR (station_id=8)
(8, 2, 22.0, 'available', 400, 32, 'ENR001', 'ABB', 'AC22', '2023-08-01', '2023-10-12', 'system', 'system', NOW(), NOW()),
(8, 4, 350.0, 'occupied', 800, 500, 'ENR002', 'Siemens', 'DC350', '2023-08-01', '2023-10-12', 'system', 'system', NOW(), NOW())
;
