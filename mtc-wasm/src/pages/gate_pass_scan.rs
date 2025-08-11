use super::*;
use wasm_bindgen_futures::JsFuture;

static MTC_QR_CODE_SUFFIX: &str = "MTC:GATE-PASS:";
static MTC_OFFLINE_MODE_KEY: &str = "mtc_offline_mode_key";
static ROUT_GATE_PASS_VALIDATION_SCANS: &str = "gate-pass-validation-scans";

#[component]
pub fn GatePassScan() -> Element {
    breadcrumbs!("menu-gate-pass-validation-scan");
    check_permission!(PERMISSION_GATE_PASS_VALIDATE);
    let qr_code_scanner_element_id = "gate_pass_validation_scanner";

    let mut last_synced_at_details_signal = use_signal(|| String::new());

    let mut offline_mode = use_local_storage(MTC_OFFLINE_MODE_KEY, || Value::Bool(false));
    let enable_offline_mode =
        move |event: Event<FormData>| offline_mode.set(Value::Bool(event.checked()));

    let mut loading_spinner_hidden_signal = use_signal(|| true);
    let sync_gate_passes = move || {
        spawn(async move {
            loading_spinner_hidden_signal.set(false);
            if let Err(error) = sync_indexed_db_gate_passes(last_synced_at_details_signal).await {
                error!("failed to sync gate passes: error={:?}", error);
            };
            loading_spinner_hidden_signal.set(true);
        });
    };

    use_effect(move || {
        spawn(async move {
            let database_signal = use_state().indexed_db();
            let database_opt = database_signal.read();
            let database_ref = database_opt.as_ref().unwrap();
            let last_synced_at_opt = get_sync_entry(database_ref, SyncEntryId::GatePassSync)
                .await
                .ok()
                .map(|sync_entry| sync_entry.last_synced_at);
            last_synced_at_details_signal
                .set(last_synced_at_details(last_synced_at_opt).to_string());

            match JsFuture::from(jsFfiCreateHtml5QrcodeScanner(&qr_code_scanner_element_id))
                .await
                .map(|value| value.as_string())
            {
                Ok(text_opt) => match text_opt {
                    Some(text) => {
                        if text.starts_with(MTC_QR_CODE_SUFFIX) {
                            let gate_pass_id = text.replace(MTC_QR_CODE_SUFFIX, "");
                            navigator().push(route!(
                                ROUT_GATE_PASS_VALIDATION_SCANS,
                                "results",
                                gate_pass_id
                            ));
                        } else {
                            navigator().push(route!(
                                ROUT_GATE_PASS_VALIDATION_SCANS,
                                "errors",
                                "message-gate-pass-validation-scan-qr-code-unknown-error"
                            ));
                        }
                    }
                    None => {
                        navigator().push(route!(
                            ROUT_GATE_PASS_VALIDATION_SCANS,
                            "errors",
                            "message-gate-pass-validation-scan-qr-code-unknown-error"
                        ));
                    }
                },
                Err(error) => {
                    navigator().push(route!(
                        ROUT_GATE_PASS_VALIDATION_SCANS,
                        "errors",
                        "message-gate-pass-validation-scan-qr-code-scan-error"
                    ));
                }
            }
        });
    });

    rsx! {
        span {
            {last_synced_at_details_signal}
        }
        div {
            style: "display: flex; justify-content: space-between;",
            div {
                style: "display: flex; align-items: flex-start;",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        sync_gate_passes();
                    },
                    span {
                        class: "loading loading-spinner",
                        hidden: loading_spinner_hidden_signal(),
                    }
                    span {
                        {t!("action-gate-pass-sync")}
                    }
                }
            }
            div {
                class: "label cursor-pointer justify-start gap-5",
                input {
                    r#type: "checkbox",
                    class: "btn",
                    initial_checked: offline_mode.get().self_bool().unwrap_or_default(),
                    onchange: enable_offline_mode,
                    "aria-label": t!("action-gate-pass-offline-mode"),
                }
            }
        }
        br {
        }
        section {
            div {
                id: qr_code_scanner_element_id,
                class: "div-centered",
            }
        }
    }
}

async fn sync_indexed_db_gate_passes(
    mut last_synced_at_details_signal: Signal<String>,
) -> Result<(), Error> {
    let database_signal = use_state().indexed_db();
    let database_opt = database_signal.read();
    let database_ref = database_opt.as_ref().unwrap();
    let now = Utc::now();
    let now_before = |date_time: Option<Cow<'static, str>>| {
        date_time.is_none_or(|date_time| {
            date_time
                .parse::<DateTime<Utc>>()
                .ok()
                .map(|date_time| date_time <= now)
                .unwrap_or(true)
        })
    };

    let mut last_synced_at_opt = get_sync_entry(database_ref, SyncEntryId::GatePassSync)
        .await
        .ok()
        .map(|sync_entry| sync_entry.last_synced_at);
    while now_before(last_synced_at_opt.clone()) {
        let request = SyncGatePassRequest {
            last_synced_at: last_synced_at_opt.clone(),
        };

        match state!(client)
            .post(url!("gate-pass-syncs"))
            .json(&request)
            .send()
            .await
            .get_value()
            .await
            .self_obj::<SyncGatePassResponse>()
        {
            Some(sync_response) => {
                process_gate_passes(database_ref, sync_response.gate_passes).await?;

                let sync_entry = SyncEntry {
                    id: SyncEntryId::GatePassSync.name(),
                    last_synced_at: sync_response.last_synced_at.clone(),
                };
                put_sync_entry(database_ref, sync_entry).await?;

                last_synced_at_opt = Some(sync_response.last_synced_at.clone());

                last_synced_at_details_signal
                    .set(last_synced_at_details(last_synced_at_opt.clone()).to_string());
            }
            None => {
                error!("failed to post sync request");
                break;
            }
        }
    }
    Ok(())
}

fn last_synced_at_details(last_synced_at_opt: Option<Cow<'static, str>>) -> Cow<'static, str> {
    let last_synced_at_str = last_synced_at_opt
        .and_then(|date_time| date_time.parse::<DateTime<Local>>().ok())
        .map(|date_time| date_time.format("%H:%M %d/%m/%Y").to_string())
        .map(|date_time_str| Cow::<String>::Owned(date_time_str))
        .unwrap_or_else(|| Cow::Owned(t!("message-gate-pass-last-sync-not-found")));
    Cow::Owned(format!(
        "{}: {last_synced_at_str}",
        t!("message-gate-pass-last-sync"),
    ))
}

#[component]
pub fn GatePassScanError(#[props(into)] error: String) -> Element {
    error_dialog!(error.as_str());
    navigator().go_back();
    rsx! {}
}

#[component]
pub fn GatePassScanResult(#[props(into)] id: String) -> Element {
    breadcrumbs!("menu-gate-pass-validation-scan-result");
    check_permission!(PERMISSION_GATE_PASS_VALIDATE);

    let id = use_memo(use_reactive!(|id| id));
    let offline_mode = use_local_storage(MTC_OFFLINE_MODE_KEY, || Value::Bool(false));
    let gate_pass_resource = use_resource(move || async move {
        get_sync_gate_pass(
            Cow::Owned(id()),
            offline_mode.get().self_bool().unwrap_or_default(),
        )
        .await
    });
    let gate_pass_opt = gate_pass_resource.suspend()?();
    let validation_result = validate_gate_pass(&gate_pass_opt);
    let allow_any_vehicle = gate_pass_opt
        .as_ref()
        .map(|gate_pass| gate_pass.allow_any_vehicle)
        .unwrap_or(false);
    rsx! {
        div {
            class: "text-center text-3xl font-bold",
            h1 {
                if validation_result == GatePassValidationResult::Valid {
                    div {
                        style: "color: green;",
                        { t!("message-gate-pass-scan-result-valid") }
                    }
                } else {
                    div {
                        style: "color: red;",
                        { t!("message-gate-pass-scan-result-invalid") }
                    }
                }
            }
            h2 {
                match validation_result {
                    GatePassValidationResult::NotFound => {
                        { t!("message-gate-pass-scan-result-not-found") }
                    },
                    GatePassValidationResult::Valid => {
                        { "".to_string() }
                    },
                    GatePassValidationResult::Deleted => {
                        { t!("message-gate-pass-scan-result-deleted") }
                    },
                    GatePassValidationResult::Expired => {
                        { t!("message-gate-pass-scan-result-expired") }
                    }
                }
            }
            h2 {
                if validation_result != GatePassValidationResult::NotFound {
                    { owner_details(&gate_pass_opt) }
                }
            }
            h2 {
                if validation_result != GatePassValidationResult::NotFound {
                    if allow_any_vehicle {
                        { allow_any_vehicle_details() }
                    } else {
                        { vehicle_details(&gate_pass_opt) }
                    }
                }
            }
        }
    }
}

fn owner_details(gate_pass_opt: &Option<SyncGatePass>) -> Option<String> {
    gate_pass_opt
        .as_ref()
        .and_then(|gate_pass| gate_pass.owner.as_ref())
        .map(|owner| {
            format!(
                "{} {} {}",
                &*owner.last_name, &*owner.first_name, &*owner.middle_name
            )
        })
}

fn vehicle_details(gate_pass_opt: &Option<SyncGatePass>) -> Option<String> {
    gate_pass_opt
        .as_ref()
        .and_then(|gate_pass| gate_pass.vehicle.as_ref())
        .map(|vehicle| {
            format!(
                "{} {} {} {} {}",
                vehicle.number_plate,
                t!(format!("gate-pass-vehicle-color-{:?}", vehicle.color)
                    .to_lowercase()
                    .as_str()),
                t!(
                    format!("gate-pass-vehicle-body-type-{:?}", vehicle.body_type)
                        .to_lowercase()
                        .as_str()
                ),
                vehicle.manufacturer,
                vehicle.model.clone().unwrap_or_default(),
            )
        })
}

fn allow_any_vehicle_details() -> Option<String> {
    Some(t!("message-gate-pass-allow-any-vehicle"))
}

async fn get_sync_gate_pass(
    gate_pass_id: Cow<'static, str>,
    offline_mode: bool,
) -> Option<SyncGatePass> {
    let gate_pass_cache_entry = if offline_mode {
        get_indexed_db_gate_pass(gate_pass_id).await
    } else {
        match state!(client)
            .get(url!(API_GATE_PASSES, gate_pass_id.as_ref(), "validations"))
            .send()
            .await
        {
            Ok(response) => response.json().await.ok().self_obj::<SyncGatePass>(),
            Err(error) => {
                if !error.status().unwrap_or_default().is_client_error() {
                    get_indexed_db_gate_pass(gate_pass_id).await
                } else {
                    None
                }
            }
        }
    };
    gate_pass_cache_entry
}

async fn get_indexed_db_gate_pass(gate_pass_id: Cow<'static, str>) -> Option<SyncGatePass> {
    let database_signal = use_state().indexed_db();
    let database_opt = database_signal.read();
    let database_ref = database_opt.as_ref().unwrap();
    get_gate_pass(database_ref, gate_pass_id).await.ok()
}

fn validate_gate_pass(gate_pass: &Option<SyncGatePass>) -> GatePassValidationResult {
    match gate_pass {
        Some(gate_pass_cache_entry) => {
            if gate_pass_cache_entry.deleted {
                return GatePassValidationResult::Deleted;
            }
            let expired = &gate_pass_cache_entry
                .expired_at
                .parse::<DateTime<Utc>>()
                .ok()
                .map(|expired_at| expired_at < Utc::now())
                .unwrap_or(true);
            if expired == &true {
                return GatePassValidationResult::Expired;
            }
            GatePassValidationResult::Valid
        }
        None => GatePassValidationResult::NotFound,
    }
}