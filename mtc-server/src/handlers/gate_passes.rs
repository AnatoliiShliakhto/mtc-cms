use super::*;
use base64::{engine::general_purpose, Engine as _};
use server_macros::handler;
use validator::Validate;

const EMPTY_STR: &'static str = "";

#[handler(permission = "gate_passes::write")]
pub async fn create_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<CreateGatePassRequest>,
) {
    info!(
        "Received create Gate Pass request: allow_any_vehicle={}",
        request.allow_any_vehicle
    );
    let user_login = session.get_auth_login().await?;
    request.validate()?;
    request.normalize();
    request.created_by = Some(user_login.clone());
    request.updated_by = Some(user_login);
    state.repository.create_gate_pass(request).await.map(Json)
}

#[handler(permission = "gate_passes::write")]
pub async fn create_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<CreateGatePassBatchRequest>,
) {
    info!(
        "Received create Gate Pass batch request: number_of_gate_passes={}",
        request.requests.len()
    );
    let user_login = session.get_auth_login().await?;
    for gate_passes_request in request.requests.iter_mut() {
        gate_passes_request.validate()?;
        gate_passes_request.normalize();
        gate_passes_request.created_by = Some(user_login.clone());
        gate_passes_request.updated_by = Some(user_login.clone());
    }
    state.repository.create_gate_passes(request).await.map(Json)
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
    request.validate()?;
    request.normalize();
    request.id = Some(gate_pass_id);
    request.updated_by = Some(user_login);
    state.repository.update_gate_pass(request).await.map(Json)
}

#[handler(permission = "gate_passes::write")]
pub async fn update_gate_pass_block_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
    Payload(mut request): Payload<UpdateGatePassBlockRequest>,
) {
    if request.block.is_some() {
        info!("Received update Gate Pass block request: gate_pass_id={gate_pass_id}");
    } else {
        info!("Received delete Gate Pass block request: gate_pass_id={gate_pass_id}");
    }
    let user_login = session.get_auth_login().await?;
    request.id = Some(gate_pass_id);
    request.updated_by = Some(user_login);
    state
        .repository
        .update_gate_pass_block(request)
        .await
        .map(Json)
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
pub async fn find_gate_passes_handler(state: State<Arc<AppState>>, session: Session) {
    info!("Received get Gate Passes request");
    state.repository.find_gate_passes().await.map(Json)
}

#[handler(permission = "gate_passes::read")]
pub async fn search_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<SearchGatePassRequest>,
) {
    info!("Received search Gate Passes request");
    request.normalize();
    state.repository.search_gate_passes(request).await.map(Json)
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
        .map(|gate_pass| SyncGatePass {
            id: gate_pass.id,
            number: gate_pass.number,
            expired_at: gate_pass.expired_at,
            deleted: gate_pass.deleted,
            owner: Some(gate_pass.owner),
            vehicles: gate_pass.vehicles,
            allow_any_vehicle: gate_pass.allow_any_vehicle,
            block: gate_pass.block,
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
        .has_permission(PERMISSION_GATE_PASS_FULL_SYNC)
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
        .vehicles
        .iter_mut()
        .for_each(|vehicle| {
            vehicle.number_plate = Cow::Borrowed(EMPTY_STR);
            vehicle.vin_code = None
        });
}

#[handler(permission = "gate_passes::write")]
pub async fn send_gate_pass_email_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
    Payload(request): Payload<SendGatePassEmailRequest>,
) {
    info!("Received send Gate Pass email request: gate_pass_id={gate_pass_id}");
    request.validate()?;
    let gate_pass = state.repository.find_gate_pass(gate_pass_id).await?;
    let logo_bytes = tokio::fs::read(format!(
        "{}/assets/favicon.ico",
        state.config.paths.www_path
    ))
    .await?;
    let qr_code_bytes = generate_qr_code_png(&gate_pass.id).unwrap_or_default();
    let gate_pass_email_html = state
        .template
        .gate_pass_email_html
        .clone()
        .replace("{{number}}", &gate_pass.number);
    let mail_request = GatePassSendMailRequest {
        sender: state.config.smtp.sender.clone(),
        recipient: request.recipient_email.clone(),
        gate_pass_email_html: Cow::Owned(gate_pass_email_html),
        logo_bytes,
        qr_code_bytes,
    };
    state.smtp_client.send_gate_pass_email(mail_request).await?;
    Ok(())
}

#[handler(permission = "gate_passes::write")]
pub async fn generate_gate_pass_front_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Path(gate_pass_id): Path<Cow<'static, str>>,
) {
    info!("Received generate Gate Pass front request: gate_pass_id={gate_pass_id}");
    let gate_pass = state.repository.find_gate_pass(gate_pass_id).await?;
    let qr_code_png_base64 = generate_qr_code_png(&gate_pass.id)
        .map(|qr_code| general_purpose::STANDARD.encode(qr_code.as_slice()))
        .unwrap_or_default();
    let gate_pass_front_html_template = state.template.gate_pass_front_html.clone();
    let gate_pass_front_html = gate_pass_front_html(
        &gate_pass,
        &qr_code_png_base64,
        &gate_pass_front_html_template,
    )?;
    Ok(Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(gate_pass_front_html)
        .unwrap())
}

fn generate_qr_code_png(gate_pass_id: &Cow<'static, str>) -> Option<Vec<u8>> {
    qrcode_generator::to_png_to_vec(
        format!("MTC:GATE-PASS:{}", gate_pass_id).as_bytes(),
        qrcode_generator::QrCodeEcc::Low,
        512,
    )
    .ok()
}

#[handler(permission = "gate_passes::write")]
pub async fn generate_gate_pass_back_handler(state: State<Arc<AppState>>, session: Session) {
    info!("Received generate Gate Pass back request");
    Ok(Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(state.template.gate_pass_back_html.clone().into_owned())
        .unwrap())
}
