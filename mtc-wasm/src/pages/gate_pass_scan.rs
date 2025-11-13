use super::*;
use indexed_db::Database;
use serde_wasm_bindgen::from_value;
use std::collections::HashMap;
use wasm_bindgen_futures::JsFuture;

static MTC_QR_CODE_SUFFIX: &str = "MTC:GATE-PASS:";
static MTC_OFFLINE_MODE_KEY: &str = "mtc_offline_mode_key";
static MTC_CAMERA_SETTINGS_KEY: &str = "mtc_camera_settings_key";
static ROUT_GATE_PASS_VALIDATION_SCANS: &str = "gate-pass-validation-scans";

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
struct CameraSettings {
    user_camera: Camera,
    environment_camera: Camera,
    selected_camera_label: String,
    torch_on: bool,
}

impl CameraSettings {
    pub fn selectedCamera(&self) -> &Camera {
        if self.selected_camera_label == self.user_camera.label {
            &self.user_camera
        } else {
            &self.environment_camera
        }
    }

    pub fn switchCamera(&mut self) {
        if self.selected_camera_label == self.user_camera.label {
            self.selected_camera_label = self.environment_camera.label.clone()
        } else {
            self.selected_camera_label = self.user_camera.label.clone()
        }
    }

    pub fn toggleCameraTorch(&mut self) {
        self.torch_on = !self.torch_on;
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Camera {
    label: String,
    torch_supported: bool,
}

#[component]
pub fn GatePassScanView() -> Element {
    breadcrumbs!("menu-gate-pass-validation-scan");
    check_permission!(PERMISSION_GATE_PASS_VALIDATE);
    let qr_code_scanner_element_id = "gate_pass_validation_scanner";

    let indexed_db_signal = use_state().indexed_db();
    let mut last_synced_at_details_signal = use_signal(|| String::new());
    let mut loading_spinner_hidden_signal = use_signal(|| true);

    let sync_gate_passes = move |event: Event<MouseData>| async move {
        loading_spinner_hidden_signal.set(false);
        if let Err(error) =
            sync_indexed_db_gate_passes(last_synced_at_details_signal, indexed_db_signal).await
        {
            error!("failed to sync gate passes: error={:?}", error);
        };
        loading_spinner_hidden_signal.set(true);
    };

    let mut offline_mode_ls = use_local_storage(MTC_OFFLINE_MODE_KEY, || Value::Bool(false));
    let mut offline_mode_signal =
        use_signal(|| offline_mode_ls.get().self_bool().unwrap_or_default());
    let switch_offline_mode = move |event: Event<MouseData>| {
        let updated_offline_mode = !offline_mode_ls.get().self_bool().unwrap_or_default();
        offline_mode_ls.set(Value::Bool(updated_offline_mode));
        offline_mode_signal.set(updated_offline_mode);
    };

    let switch_camera = move |event: Event<MouseData>| async move {
        let mut camera_settings_ls = use_camera_settings_local_storage().await;
        let mut camera_settings =
            serde_json::from_value::<CameraSettings>(camera_settings_ls.get()).unwrap_or_default();
        camera_settings.switchCamera();
        camera_settings_ls.set(serde_json::to_value(camera_settings).unwrap_or_default());
        navigator().push(route!(API_ADMINISTRATOR, "loops"));
    };

    let mut camera_torch_on_signal = use_signal(|| false);
    let mut camera_torch_supported_signal = use_signal(|| false);
    let toggle_camera_torch = move |event: Event<MouseData>| async move {
        let mut camera_settings_ls = use_camera_settings_local_storage().await;
        let mut camera_settings =
            serde_json::from_value::<CameraSettings>(camera_settings_ls.get()).unwrap_or_default();
        camera_settings.toggleCameraTorch();
        let torch_on = camera_settings.torch_on;
        camera_torch_on_signal.set(torch_on);
        camera_settings_ls.set(serde_json::to_value(camera_settings).unwrap_or_default());
        if JsFuture::from(jsFfiToggleHtml5QrcodeScannerTorch(torch_on))
            .await
            .is_err()
        {
            error!("failed to invoke jsFfiToggleHtml5QrcodeScannerTorch");
        }
    };

    use_effect(move || {
        spawn(async move {
            let indexed_db_opt = indexed_db_signal.read();
            let indexed_db_ref = indexed_db_opt.as_ref().unwrap();
            let last_synced_at_opt = get_sync_entry(indexed_db_ref, SyncEntryId::GatePassSync)
                .await
                .ok()
                .map(|sync_entry| sync_entry.last_synced_at);
            last_synced_at_details_signal
                .set(last_synced_at_details(last_synced_at_opt).to_string());

            let camera_settings_ls = use_camera_settings_local_storage().await;
            let camera_settings_opt =
                serde_json::from_value::<CameraSettings>(camera_settings_ls.get()).ok();
            if let Some(camera_settings) = camera_settings_opt {
                camera_torch_on_signal.set(camera_settings.torch_on);
                camera_torch_supported_signal.set(camera_settings.selectedCamera().torch_supported);
                scan_gate_pass(
                    qr_code_scanner_element_id,
                    camera_settings.selectedCamera(),
                    camera_settings.torch_on,
                )
                .await;
            }
        });
    });

    rsx! {
        span {
            style: "display: block; text-align: center;",
            {last_synced_at_details_signal}
        }
        div {
            id: qr_code_scanner_element_id,
            style: "max-width: 250px; max-heigh: 250px; margin-left: auto; margin-right: auto;",
        }
        div { style: "display: flex; justify-content: center;",
            button {
                class: "btn btn-xl btn-ghost",
                title: t!("gate-pass-action-sync"),
                onclick: sync_gate_passes,
                if loading_spinner_hidden_signal() {
                    Icon { icon: Icons::Refresh, class: "size-8" }
                } else {
                    span {
                        class: "loading loading-spinner",
                        hidden: loading_spinner_hidden_signal(),
                    }
                }
            }
            button {
                class: "btn btn-xl btn-ghost",
                title: t!("gate-pass-action-offline-mode"),
                onclick: switch_offline_mode,
                if offline_mode_signal() {
                    Icon { icon: Icons::Offline, class: "size-8" }
                } else {
                    Icon { icon: Icons::Online, class: "size-8" }
                }
            }
            button {
                class: "btn btn-xl btn-ghost",
                title: t!("gate-pass-action-switch-camera"),
                onclick: switch_camera,
                Icon { icon: Icons::PhoneRotation, class: "size-8" }
            }
            button {
                class: "btn btn-xl btn-ghost",
                title: t!("gate-pass-action-toggle-camera-flash"),
                hidden: !camera_torch_supported_signal(),
                onclick: toggle_camera_torch,
                if camera_torch_on_signal() {
                    Icon { icon: Icons::PhoneFlashOn, class: "size-8" }
                } else {
                    Icon { icon: Icons::PhoneFlashOff, class: "size-8" }
                }
            }
        }
    }
}

async fn use_camera_settings_local_storage() -> UseLocalStorage {
    let mut camera_settings_ls = use_local_storage(MTC_CAMERA_SETTINGS_KEY, || Value::Null);
    if !validate_camera_settings(camera_settings_ls).await {
        if let Some(camera_settings) = init_camera_settings().await {
            camera_settings_ls.set(serde_json::to_value(camera_settings).unwrap_or_default());
        }
    }
    camera_settings_ls
}

/// Validates camera_settings: settings are valid if not null and all camera labels are still supported.
async fn validate_camera_settings(camera_settings_ls: UseLocalStorage) -> bool {
    let camera_settings = camera_settings_ls.get();
    if camera_settings.is_null() {
        false
    } else {
        let supported_camera_labels = supported_camera_label_to_id().await;
        let camera_settings = serde_json::from_value::<CameraSettings>(camera_settings_ls.get())
            .ok()
            .unwrap_or_default();
        supported_camera_labels.contains_key(&camera_settings.selected_camera_label)
            && supported_camera_labels.contains_key(&camera_settings.user_camera.label)
            && supported_camera_labels.contains_key(&camera_settings.environment_camera.label)
    }
}

async fn init_camera_settings() -> Option<CameraSettings> {
    match JsFuture::from(jsFfiDetectUserEnvironmentHtml5QrcodeCameras())
        .await
        .map(|value| from_value::<Vec<Camera>>(value).ok())
    {
        Ok(user_environment_cameras_opt) => {
            user_environment_cameras_opt.map(|user_environment_cameras| CameraSettings {
                user_camera: user_environment_cameras.get(0).unwrap().clone(),
                environment_camera: user_environment_cameras.get(1).unwrap().clone(),
                selected_camera_label: user_environment_cameras.get(1).unwrap().clone().label,
                torch_on: false,
            })
        }
        Err(error) => {
            error!("failed to detect camera ids: {:?}", error);
            None
        }
    }
}

async fn sync_indexed_db_gate_passes(
    mut last_synced_at_details_signal: Signal<String>,
    indexed_db_signal: Signal<Option<Database<Error>>>,
) -> Result<(), Error> {
    let database_opt = indexed_db_signal.read();
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
            .post(url!(API_GATE_PASSES, "syncs"))
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
        .and_then(|date_time| date_time.parse::<DateTime<Utc>>().ok())
        .map(|dt_utc| DateTime::<Local>::from(dt_utc))
        .map(|date_time| date_time.format("%H:%M %d/%m/%Y").to_string())
        .map(|date_time_str| Cow::<String>::Owned(date_time_str))
        .unwrap_or_else(|| Cow::Owned(t!("gate-pass-message-last-sync-not-found")));
    Cow::Owned(format!(
        "{}: {last_synced_at_str}",
        t!("gate-pass-message-last-sync"),
    ))
}

async fn scan_gate_pass(qr_code_scanner_element_id: &str, camera: &Camera, torch_on: bool) {
    let camera_id = supported_camera_label_to_id()
        .await
        .get(&camera.label)
        .map(|camera_id| camera_id.to_string())
        .unwrap_or_default();
    match JsFuture::from(jsFfiCreateHtml5QrcodeScanner(
        qr_code_scanner_element_id,
        &camera_id,
        torch_on,
    ))
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
                        "gate-pass-message-validation-scan-qr-code-unknown-error"
                    ));
                }
            }
            None => {
                navigator().push(route!(
                    ROUT_GATE_PASS_VALIDATION_SCANS,
                    "errors",
                    "gate-pass-message-validation-scan-qr-code-unknown-error"
                ));
            }
        },
        Err(error) => {
            navigator().push(route!(
                ROUT_GATE_PASS_VALIDATION_SCANS,
                "errors",
                "gate-pass-message-validation-scan-qr-code-scan-error"
            ));
        }
    }
}

async fn supported_camera_label_to_id() -> HashMap<String, String> {
    JsFuture::from(jsFfiSupportedCameraLabelToId())
        .await
        .map(|value| from_value::<HashMap<String, String>>(value).unwrap_or_default())
        .unwrap_or_default()
}

#[component]
pub fn GatePassScanErrorView(#[props(into)] error: String) -> Element {
    error_dialog!(error.as_str());
    navigator().go_back();
    rsx! {}
}

#[component]
pub fn GatePassScanResultView(#[props(into)] id: String) -> Element {
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
    let valid = validation_result == GatePassValidationResult::Valid;
    let deleted = validation_result == GatePassValidationResult::Deleted;
    let validation_result_not_found = validation_result == GatePassValidationResult::NotFound;
    let allow_any_vehicle = gate_pass_opt
        .as_ref()
        .map(|gate_pass| gate_pass.allow_any_vehicle)
        .unwrap_or(false);
    rsx! {
        div { class: "text-center text-3xl font-bold",
            h1 {
                if GatePassValidationResult::Valid == validation_result {
                    div { style: "color: green;", {t!("gate-pass-message-scan-result-valid")} }
                } else if let GatePassValidationResult::Blocked(ref reason) = validation_result {
                    div { style: "color: red;", {t!("gate-pass-message-scan-result-blocked")} }
                } else {
                    div { style: "color: red;", {t!("gate-pass-message-scan-result-invalid")} }
                }
            }
            h2 {
                match validation_result {
                    GatePassValidationResult::Valid => "".to_string(),
                    GatePassValidationResult::Blocked(ref reason) => reason.to_string(),
                    GatePassValidationResult::Expired => t!("gate-pass-message-scan-result-expired"),
                    GatePassValidationResult::Deleted => t!("gate-pass-message-scan-result-deleted"),
                    GatePassValidationResult::NotFound => {
                        t!("gate-pass-message-scan-result-not-found")
                    }
                }
            }
            h2 {
                if !deleted && !validation_result_not_found {
                    {owner_details(&gate_pass_opt)}
                }
            }
            h2 {
                if !validation_result_not_found {
                    if valid && allow_any_vehicle {
                        {allow_any_vehicle_details()}
                    } else if !deleted {
                        {vehicle_details(&gate_pass_opt)}
                    }                }
            }
            button {
                class: "btn btn-xl btn-ghost",
                onclick: move |_| {
                    navigator().go_back();
                },
                Icon { icon: Icons::QrPhoneScan, class: "size-10" }
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
        .and_then(|gate_pass| gate_pass.vehicles.first())
        .map(|vehicle| {
            format!(
                "{} {} {} {} {}",
                vehicle.number_plate,
                gate_pass_vehicle_color_name(&vehicle.color),
                gate_pass_vehicle_body_type_name(&vehicle.body_type),
                vehicle.manufacturer,
                vehicle.model.clone().unwrap_or_default(),
            )
        })
}

fn allow_any_vehicle_details() -> Option<String> {
    Some(t!("gate-pass-message-allow-any-vehicle"))
}

async fn get_sync_gate_pass(
    gate_pass_id: Cow<'static, str>,
    offline_mode: bool,
) -> Option<SyncGatePass> {
    let indexed_db_signal = use_state().indexed_db();
    let gate_pass_cache_entry = if offline_mode {
        get_indexed_db_gate_pass(gate_pass_id, indexed_db_signal).await
    } else {
        match state!(client)
            .get(url!(API_GATE_PASSES, gate_pass_id.as_ref(), "validations"))
            .send()
            .await
        {
            Ok(response) => response.json().await.ok().self_obj::<SyncGatePass>(),
            Err(error) => {
                if !error.status().unwrap_or_default().is_client_error() {
                    get_indexed_db_gate_pass(gate_pass_id, indexed_db_signal).await
                } else {
                    None
                }
            }
        }
    };
    gate_pass_cache_entry
}

async fn get_indexed_db_gate_pass(
    gate_pass_id: Cow<'static, str>,
    indexed_db_signal: Signal<Option<Database<Error>>>,
) -> Option<SyncGatePass> {
    let indexed_db_opt = indexed_db_signal.read();
    let indexed_db_ref = indexed_db_opt.as_ref().unwrap();
    get_gate_pass(indexed_db_ref, gate_pass_id).await.ok()
}

fn validate_gate_pass(gate_pass: &Option<SyncGatePass>) -> GatePassValidationResult {
    match gate_pass {
        Some(sync_gate_pass) => {
            if sync_gate_pass.deleted {
                return GatePassValidationResult::Deleted;
            }
            if sync_gate_pass.expired() == true {
                return GatePassValidationResult::Expired;
            }
            if sync_gate_pass.blocked() == true {
                let reason = gate_pass
                    .as_ref()
                    .and_then(|gate_pass| gate_pass.block.as_ref())
                    .map(|block| block.reason.clone())
                    .unwrap_or_default();
                return GatePassValidationResult::Blocked(reason);
            }
            GatePassValidationResult::Valid
        }
        None => GatePassValidationResult::NotFound,
    }
}
