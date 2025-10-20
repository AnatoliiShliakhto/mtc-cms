BEGIN TRANSACTION;

DELETE FROM gate_passes;

REMOVE FIELD IF EXISTS vehicles.*.vin_code ON TABLE gate_passes;
DEFINE FIELD vehicles.*.vin_code ON TABLE gate_passes TYPE option<string>;
REMOVE FIELD IF EXISTS vehicles.*.model ON TABLE gate_passes;
DEFINE FIELD vehicles.*.model ON TABLE gate_passes TYPE option<string>;

REMOVE INDEX IF EXISTS gate_passes_vehicle_number_plate_index ON TABLE gate_passes;
DEFINE INDEX IF NOT EXISTS gate_passes_vehicle_number_plate_unique_index ON TABLE gate_passes FIELDS vehicles.*.number_plate UNIQUE;

COMMIT TRANSACTION;