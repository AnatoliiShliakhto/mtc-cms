BEGIN TRANSACTION;

REMOVE TABLE IF EXISTS sequences;
DEFINE TABLE sequences SCHEMALESS;
CREATE sequences:gate_pass_numbers CONTENT {
    next: 1,
};

REMOVE FUNCTION IF EXISTS fn::sequence_next;
DEFINE FUNCTION fn::sequence_next($sequence_name: string) {
    LET $new_value = (UPDATE type::thing('sequences', $sequence_name) SET next += 1 RETURN next)[0];
    RETURN $new_value.next - 1;
};

REMOVE TABLE IF EXISTS gate_passes;
DEFINE TABLE gate_passes SCHEMAFULL;
DEFINE FIELD number ON TABLE gate_passes TYPE string;
DEFINE FIELD expired_at ON TABLE gate_passes TYPE datetime;
DEFINE FIELD deleted ON TABLE gate_passes TYPE bool;
DEFINE FIELD owner ON TABLE gate_passes FLEXIBLE TYPE object;
DEFINE FIELD owner.first_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.middle_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.last_name ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.title ON TABLE gate_passes TYPE string;
DEFINE FIELD owner.unit ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles ON TABLE gate_passes TYPE array<object> DEFAULT [];
DEFINE FIELD vehicles.*.number_plate ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles.*.vin_code ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles.*.manufacturer ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles.*.model ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles.*.color ON TABLE gate_passes TYPE string;
DEFINE FIELD vehicles.*.body_type ON TABLE gate_passes TYPE string;
DEFINE FIELD allow_any_vehicle ON TABLE gate_passes TYPE bool;
DEFINE FIELD block ON TABLE gate_passes FLEXIBLE TYPE option<object>;
DEFINE FIELD block.expired_at ON TABLE gate_passes TYPE datetime;
DEFINE FIELD block.reason ON TABLE gate_passes TYPE string;
DEFINE FIELD created_at ON TABLE gate_passes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE gate_passes TYPE datetime VALUE time::now();
DEFINE FIELD created_by ON TABLE gate_passes TYPE string;
DEFINE FIELD updated_by ON TABLE gate_passes TYPE string;

DEFINE INDEX gate_passes_updated_by_index ON gate_passes FIELDS updated_at;
DEFINE INDEX gate_passes_owner_last_name_index ON gate_passes FIELDS owner.last_name;
DEFINE INDEX gate_passes_vehicle_number_plate_index ON TABLE gate_passes FIELDS vehicles.*.number_plate;

CREATE permissions CONTENT {
    id: 'gate_passes_read',
    slug: 'gate_passes::read',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'gate_passes_write',
    slug: 'gate_passes::write',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'gate_passes_delete',
    slug: 'gate_passes::delete',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'gate_passes_sync',
    slug: 'gate_passes::sync',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'gate_passes_full_sync',
    slug: 'gate_passes::full_sync',
    created_by: $login
};
CREATE permissions CONTENT {
    id: 'gate_passes_validate',
    slug: 'gate_passes::validate',
    created_by: $login
};

RELATE roles:administrator->role_permissions->permissions:gate_passes_read;
RELATE roles:administrator->role_permissions->permissions:gate_passes_write;
RELATE roles:administrator->role_permissions->permissions:gate_passes_delete;
RELATE roles:administrator->role_permissions->permissions:gate_passes_sync;
RELATE roles:administrator->role_permissions->permissions:gate_passes_full_sync;
RELATE roles:administrator->role_permissions->permissions:gate_passes_validate;

COMMIT TRANSACTION;