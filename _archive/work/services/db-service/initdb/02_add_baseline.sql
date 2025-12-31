-- Insert common EV connector types
INSERT INTO connector_types (
    name, 
    description, 
    standard, 
    current_type, 
    typical_power_kw, 
    pin_configuration, 
    is_public_standard, 
    created_by,
    updated_by
) VALUES 
('Type 1 (J1772)', 'Standard North American AC connector', 'SAE J1772', 'AC', 7.2, '5-pin', TRUE, 'system', 'system'),
('Type 2 (Mennekes)', 'Standard European AC connector', 'IEC 62196-2', 'AC', 22.0, '7-pin', TRUE, 'system', 'system'),
('CCS1 (Combo 1)', 'North American DC fast charging combo connector', 'IEC 62196-3', 'DC', 350.0, 'Combo (AC+DC)', TRUE, 'system', 'system'),
('CCS2 (Combo 2)', 'European DC fast charging combo connector', 'IEC 62196-3', 'DC', 350.0, 'Combo (AC+DC)', TRUE, 'system', 'system'),
('CHAdeMO', 'Japanese DC fast charging standard', 'CHAdeMO', 'DC', 200.0, '10-pin', TRUE, 'system', 'system'),
('Tesla Supercharger', 'Tesla proprietary DC fast charging connector', 'Tesla', 'DC', 250.0, 'Proprietary', FALSE, 'system', 'system'),
('GB/T AC', 'Chinese standard AC connector', 'GB/T 20234.2', 'AC', 7.0, '7-pin', TRUE, 'system', 'system'),
('GB/T DC', 'Chinese standard DC fast charging connector', 'GB/T 20234.3', 'DC', 237.5, '9-pin', TRUE, 'system', 'system'),
('Three-Phase AC', 'High-power three-phase AC charging', 'IEC 62196-2', 'AC', 43.0, '7-pin', TRUE, 'system', 'system'),
('NACS (Tesla)', 'North American Charging Standard (formerly Tesla connector)', 'SAE J3400', 'DC', 250.0, 'Simple 2-pin', TRUE, 'system', 'system')
ON CONFLICT (name) DO UPDATE SET
    description = EXCLUDED.description,
    standard = EXCLUDED.standard,
    current_type = EXCLUDED.current_type,
    typical_power_kw = EXCLUDED.typical_power_kw;
