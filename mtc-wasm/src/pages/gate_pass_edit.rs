use super::*;

#[component]
pub fn GatePassEdit(#[props(into)] id: String) -> Element {
    breadcrumbs!("menu-gate-passes");
    check_permission!(PERMISSION_GATE_PASS_READ);

    let id = use_memo(use_reactive!(|id| id));
    let new_gate_pass = use_memo(move || id().eq(ID_CREATE));
    let gate_pass_value = if new_gate_pass() {
        json!({})
    } else {
        let resource = value_future!(url!(API_GATE_PASSES, &id()));
        let existing_quiz = resource.suspend()?;
        check_response!(existing_quiz, resource);
        existing_quiz()
    };

    rsx! {
        GatePassEditorView {id, new_gate_pass, gate_pass_value}
    }
}

#[component]
fn GatePassEditorView(
    id: Memo<String>,
    new_gate_pass: Memo<bool>,
    gate_pass_value: Value,
) -> Element {
    let body_types = VehicleBodyType::values()
        .into_iter()
        .map(|body_type| {
            (
                format!("{:?}", body_type),
                format!("gate-pass-vehicle-body-type-{:?}", body_type).to_lowercase(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let colors = VehicleColor::values()
        .into_iter()
        .map(|color| {
            (
                format!("{:?}", color),
                format!("gate-pass-vehicle-color-{:?}", color).to_lowercase(),
            )
        })
        .collect::<Vec<(String, String)>>();
    let owner = gate_pass_value.get("owner").unwrap_or_default();
    let vehicle = gate_pass_value.get("vehicle").unwrap_or_default();
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
            "vehicle": vehicle,
            "allow_any_vehicle": event.get_bool("allow_any_vehicle"),
        });

        spawn(async move {
            let request_result = if new_gate_pass() {
                post_request!(url!(API_GATE_PASSES), payload)
            } else {
                patch_request!(url!(API_GATE_PASSES, &id()), payload)
            };
            if request_result {
                navigator().replace(route!(API_ADMINISTRATOR, API_GATE_PASSES));
            }
        });
    };

    let on_delete_gate_pass = move |_| {
        spawn(async move {
            if delete_request!(url!(API_GATE_PASSES, &id())) {
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
                disabled: Some(!new_gate_pass()),
                initial_value:
                // date substring in format yyyy-mm-dd
                gate_pass_value.key_string("expired_at").map(|expired_at| expired_at[0..10].to_string()),
            }
            // owner
            FormTextField {
                name: "owner_last_name",
                title: "gate-pass-field-owner-last-name",
                required: true,
                disabled: Some(!new_gate_pass()),
                initial_value: owner.key_string("last_name"),
            }
            FormTextField {
                name: "owner_first_name",
                title: "gate-pass-field-owner-first-name",
                required: true,
                disabled: Some(!new_gate_pass()),
                initial_value: owner.key_string("first_name"),
            }
            FormTextField {
                name: "owner_middle_name",
                title: "gate-pass-field-owner-middle-name",
                required: true,
                disabled: Some(!new_gate_pass()),
                initial_value: owner.key_string("middle_name"),
            }
            FormTextField {
                name: "owner_title",
                title: "gate-pass-field-owner-title",
                required: true,
                initial_value: owner.key_string("title"),
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
                disabled: Some(!new_gate_pass()),
                initial_value: vehicle.key_string("number_plate"),
            }
            FormTextField {
                name: "vehicle_vin_code",
                title: "gate-pass-field-vehicle-vin-code",
                required: false,
                disabled: Some(!new_gate_pass()),
                initial_value: vehicle.key_string("vin_code"),
            }
            FormTextField {
                name: "vehicle_manufacturer",
                title: "gate-pass-field-vehicle-manufacturer",
                required: true,
                disabled: Some(!new_gate_pass()),
                initial_value: vehicle.key_string("manufacturer"),
            }
            FormTextField {
                name: "vehicle_model",
                title: "gate-pass-field-vehicle-model",
                required: false,
                disabled: Some(!new_gate_pass()),
                initial_value: vehicle.key_string("model"),
            }
            FormSimpleSelectField {
                name: "vehicle_color",
                title: "gate-pass-field-vehicle-color",
                required: true,
                disabled: Some(!new_gate_pass()),
                selected: vehicle.key_string("color").unwrap_or_default(),
                items: colors,
            }
            FormSimpleSelectField {
                name: "vehicle_body_type",
                title: "gate-pass-field-vehicle-body-type",
                required: true,
                disabled: Some(!new_gate_pass()),
                selected: vehicle.key_string("body_type").unwrap_or_default(),
                items: body_types,
            }
            FormCheckBoxField {
                name: "allow_any_vehicle",
                title: "gate-pass-field-allow-any-vehicle",
                disabled: Some(!new_gate_pass()),
                initial_checked: gate_pass_value.key_bool("allow_any_vehicle")
            }
        }
        EntryInfoBox {
            created_by: gate_pass_value.key_string("created_by"),
            created_at: gate_pass_value.key_datetime("created_at"),
            updated_by: gate_pass_value.key_string("updated_by"),
            updated_at: gate_pass_value.key_datetime("updated_at"),
        }
        if new_gate_pass() {
            EditorActions { form: "gate-pass-edit-form", permission: PERMISSION_GATE_PASS_WRITE }
        } else {
            EditorActions {
                form: "gate-pass-edit-form",
                delete_handler: on_delete_gate_pass,
                permission: PERMISSION_GATE_PASS_WRITE,
            }
        }
    }
}
