use super::*;
use chrono::{Datelike, NaiveDate};

#[component]
pub fn GatePassEdit(#[props(into)] id: String) -> Element {
    breadcrumbs!("menu-gate-passes");
    check_permission!(PERMISSION_GATE_PASS_READ);

    let id_memo = use_memo(use_reactive!(|id| id));
    let new_gate_pass_memo = use_memo(move || id_memo().eq(ID_CREATE));
    let gate_pass_value = if new_gate_pass_memo() {
        json!({})
    } else {
        let resource = value_future!(url!(API_GATE_PASSES, &id_memo()));
        let existing_quiz = resource.suspend()?;
        check_response!(existing_quiz, resource);
        existing_quiz()
    };

    rsx! {
        GatePassEditorView { id_memo, new_gate_pass_memo, gate_pass_value }
    }
}

#[component]
fn GatePassEditorView(
    id_memo: Memo<String>,
    new_gate_pass_memo: Memo<bool>,
    gate_pass_value: Value,
) -> Element {
    let titles = GatePassOwnerTitle::values()
        .into_iter()
        .map(|title| (format!("{:?}", title), gate_pass_owner_title_name(&title)))
        .collect::<Vec<(String, String)>>();

    let body_types = VehicleBodyType::values()
        .into_iter()
        .map(|body_type| {
            (
                format!("{:?}", body_type),
                gate_pass_vehicle_body_type_name(&body_type),
            )
        })
        .collect::<Vec<(String, String)>>();

    let colors = VehicleColor::values()
        .into_iter()
        .map(|color| (format!("{:?}", color), gate_pass_vehicle_color_name(&color)))
        .collect::<Vec<(String, String)>>();

    let owner = gate_pass_value.get("owner").unwrap_or_default();
    let vehicle = gate_pass_value
        .get("vehicles")
        .and_then(|vehicles| vehicles.as_array())
        .and_then(|vehicles| vehicles.first())
        .unwrap_or_default();

    let on_submit_gate_pass = move |event: Event<FormData>| {
        let owner = json!({
            "first_name": event.get_str("owner_first_name"),
            "middle_name": event.get_str("owner_middle_name"),
            "last_name": event.get_str("owner_last_name"),
            "title": event.get_str("owner_title"),
            "unit": event.get_str("owner_unit"),
        });
        let vehicle = json!({
            "number_plate": event.get_str("vehicle_number_plate"),
            "vin_code": event.get_str("vehicle_vin_code"),
            "manufacturer": event.get_str("vehicle_manufacturer"),
            "model": event.get_str("vehicle_model"),
            "color": event.get_str("vehicle_color"),
            "body_type": event.get_str("vehicle_body_type"),
        });
        let payload = json!({
            "id": event.get_str("id"),
            "expired_at": event.get_str("expired_at"),
            "owner": owner,
            "vehicles": vec!(vehicle),
            "allow_any_vehicle": event.get_bool("allow_any_vehicle"),
        });

        spawn(async move {
            let request_result = if new_gate_pass_memo() {
                post_request!(url!(API_GATE_PASSES), payload)
            } else {
                patch_request!(url!(API_GATE_PASSES, &id_memo()), payload)
            };
        });
    };

    let on_delete_gate_pass = move |_| {
        spawn(async move {
            if delete_request!(url!(API_GATE_PASSES, &id_memo())) {
                navigator().replace(route!(API_ADMINISTRATOR, API_GATE_PASSES));
            }
        });
    };

    rsx! {
        form {
            class: "flex grow flex-col items-center gap-3",
            id: "gate-pass-edit-form",
            autocomplete: "off",
            onsubmit: on_submit_gate_pass,
            input {
                r#type: "hidden",
                name: "id",
                initial_value: gate_pass_value.key_string("id"),
            }
            FormDateField {
                name: "expired_at",
                title: "gate-pass-field-expired-at",
                required: true,
                initial_value: gate_pass_value
                    .key_string("expired_at")
                    .map(|expired_at| expired_at[0..10].to_string())
                .or(default_expired_at()),
            }
            // owner
            FormTextField {
                name: "owner_last_name",
                title: "gate-pass-field-owner-last-name",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: owner.key_string("last_name"),
            }
            FormTextField {
                name: "owner_first_name",
                title: "gate-pass-field-owner-first-name",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: owner.key_string("first_name"),
            }
            FormTextField {
                name: "owner_middle_name",
                title: "gate-pass-field-owner-middle-name",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: owner.key_string("middle_name"),
            }
            FormSimpleSelectField {
                name: "owner_title",
                title: "gate-pass-field-owner-title",
                required: true,
                selected: owner.key_string("title").unwrap_or_default(),
                items: titles,
            }
            FormTextField {
                name: "owner_unit",
                title: "gate-pass-field-owner-unit",
                required: true,
                initial_value: owner.key_string("unit"),
            }

            // vehicle
            FormTextField {
                name: "vehicle_number_plate",
                title: "gate-pass-field-vehicle-number-plate",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: vehicle.key_string("number_plate"),
            }
            FormTextField {
                name: "vehicle_vin_code",
                title: "gate-pass-field-vehicle-vin-code",
                required: false,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: vehicle.key_string("vin_code"),
            }
            FormTextField {
                name: "vehicle_manufacturer",
                title: "gate-pass-field-vehicle-manufacturer",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: vehicle.key_string("manufacturer"),
            }
            FormTextField {
                name: "vehicle_model",
                title: "gate-pass-field-vehicle-model",
                required: false,
                disabled: Some(!new_gate_pass_memo()),
                initial_value: vehicle.key_string("model"),
            }
            FormSimpleSelectField {
                name: "vehicle_color",
                title: "gate-pass-field-vehicle-color",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                selected: vehicle.key_string("color").unwrap_or_default(),
                items: colors,
            }
            FormSimpleSelectField {
                name: "vehicle_body_type",
                title: "gate-pass-field-vehicle-body-type",
                required: true,
                disabled: Some(!new_gate_pass_memo()),
                selected: vehicle.key_string("body_type").unwrap_or_default(),
                items: body_types,
            }
            FormCheckBoxField {
                name: "allow_any_vehicle",
                title: "gate-pass-field-allow-any-vehicle",
                disabled: Some(!new_gate_pass_memo()),
                initial_checked: gate_pass_value.key_bool("allow_any_vehicle"),
            }
        }
        EntryInfoBox {
            created_by: gate_pass_value.key_string("created_by"),
            created_at: gate_pass_value.key_datetime("created_at"),
            updated_by: gate_pass_value.key_string("updated_by"),
            updated_at: gate_pass_value.key_datetime("updated_at"),
        }
        if new_gate_pass_memo() {
            EditorActions {
                form: "gate-pass-edit-form",
                permission: PERMISSION_GATE_PASS_WRITE,
            }
        } else {
            EditorActions {
                form: "gate-pass-edit-form",
                delete_handler: on_delete_gate_pass,
                permission: PERMISSION_GATE_PASS_WRITE,
            }
        }
    }
}
