use super::*;
use server_macros::handler;

const EMPTY_STR: &'static str = "";

#[handler(permission = "gate_passes::write")]
pub async fn create_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<CreateGatePassRequest>,
) {
    info!(
        "Received create Gate Pass request: vehicle_manufacturer={}, allow_any_vehicle={}",
        request.vehicle.manufacturer, request.allow_any_vehicle
    );
    let user_login = session.get_auth_login().await?;
    request.created_by = Some(user_login.clone());
    request.updated_by = Some(user_login);
    request.normalize();
    state.repository.create_gate_pass(request).await.map(Json)
}

#[handler(permission = "gate_passes::write")]
pub async fn update_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
    Payload(mut request): Payload<UpdateGatePassRequest>,
) {
    info!("Received update Gate Pass request: gate_pass_id={gate_pass_id}");
    let user_login = session.get_auth_login().await?;
    request.id = Some(gate_pass_id);
    request.updated_by = Some(user_login);
    request.normalize();
    state.repository.update_gate_pass(request).await.map(Json)
}

#[handler(permission = "gate_passes::delete")]
pub async fn delete_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
) {
    info!("Received delete Gate Pass request: gate_pass_id={gate_pass_id}");
    state
        .repository
        .delete_gate_pass(gate_pass_id)
        .await
        .map(Json)
}

#[handler(permission = "gate_passes::read")]
pub async fn find_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
) {
    info!("Received get Gate Pass request: gate_pass_id={gate_pass_id}");
    state
        .repository
        .find_gate_pass(gate_pass_id)
        .await
        .map(Json)
}

#[handler(permission = "gate_passes::read")]
pub async fn find_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<SearchGatePassRequest>,
) {
    info!("Received search Gate Pass request");
    request.normalize();
    state.repository.find_gate_passes(request).await.map(Json)
}

#[handler(permission = "gate_passes::validate")]
pub async fn find_validation_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
) {
    info!("Received validate Gate Pass request: gate_pass_id={gate_pass_id}");
    state
        .repository
        .find_gate_pass(gate_pass_id)
        .await
        .map(|mut gate_pass| SyncGatePass {
            id: gate_pass.id,
            expired_at: gate_pass.expired_at,
            deleted: gate_pass.deleted,
            owner: Some(gate_pass.owner),
            vehicle: Some(gate_pass.vehicle),
            allow_any_vehicle: gate_pass.allow_any_vehicle,
        })
        .map(Json)
}

#[handler(permission = "gate_passes::sync")]
pub async fn find_sync_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(request): Payload<SyncGatePassRequest>,
) {
    info!(
        "Received sync Gate Pass request: last_synced_at={:?}",
        request.last_synced_at
    );
    let full_sync = session
        .has_permission("gate_passes::full_sync")
        .await
        .is_ok();
    state
        .repository
        .find_sync_gate_passes(request)
        .await
        .map(|mut sync_gate_pass_response| {
            if !full_sync {
                sync_gate_pass_response
                    .gate_passes
                    .iter_mut()
                    .for_each(|gate_pass| {
                        erase_sensitive_data(gate_pass);
                    });
            }
            sync_gate_pass_response
        })
        .map(Json)
}

fn erase_sensitive_data(gate_pass: &mut SyncGatePass) {
    gate_pass.owner = None;
    gate_pass
        .vehicle
        .as_mut()
        .map(|vehicle| vehicle.number_plate = Cow::Borrowed(EMPTY_STR));
}

#[handler(permission = "gate_passes::write")]
pub async fn generate_gate_pass_qr_code_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
) {
    info!("Received generate Gate Pass qr code request: gate_pass_id={gate_pass_id}");
    state
        .repository
        .find_gate_pass(gate_pass_id)
        .await
        .map(|gate_pass| {
            qrcode_generator::to_svg_to_string(
                format!("MTC:GATE-PASS:{}", gate_pass.id).as_bytes(),
                qrcode_generator::QrCodeEcc::Low,
                512,
                None::<&str>,
            )
            .unwrap_or_default()
        })
        .map(|qr_code| {
            Response::builder()
                .header(CONTENT_TYPE, "image/svg+xml")
                .body(qr_code)
                .unwrap()
        })
}
