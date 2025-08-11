BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS gate_passes;
DEFINE TABLE gate_passes SCHEMAFULL;
DEFINE FIELD expired_at ON TABLE gate_passes TYPE datetime;
DEFINE FIELD deleted ON TABLE gate_passes TYPE bool;
DEFINE FIELD owner ON TABLE gate_passes FLEXIBLE TYPE object;
DEFINE FIELD owner.first_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.middle_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.last_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.title ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.unit ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle ON TABLE gate_passes FLEXIBLE TYPE object;
DEFINE FIELD vehicle.number_plate ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle.vin_code ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle.manufacturer ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle.model ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle.color ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicle.body_type ON TABLE gate_passes TYPE string;
DEFINE FIELD created_at ON TABLE gate_passes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE gate_passes TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE gate_passes TYPE string;
DEFINE FIELD updated_by ON TABLE gate_passes TYPE string;

COMMIT TRANSACTION;