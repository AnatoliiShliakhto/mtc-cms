use super::*;
use server_macros::handler;

// #[handler(permission = "quizzes::write")]
#[handler]
pub async fn create_gate_pass_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<CreateGatePassRequest>,
) {
    let user_login = session.get_auth_login().await?;
    request.created_by = Some(user_login.clone());
    request.updated_by = Some(user_login);
    state.repository.create_gate_pass(request).await.map(Json)
}

// #[handler(permission = "quizzes::write")]
#[handler]
pub async fn update_gate_pass_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<UpdateGatePassRequest>,
) {
    let user_login = session.get_auth_login().await?;
    request.id = Some(id);
    request.updated_by = Some(user_login);
    state.repository.update_gate_pass(request).await.map(Json)
}

// #[handler(session, permission = "quizzes::delete")]
#[handler]
pub async fn delete_gate_pass_handler(
    Path(gate_pass_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .delete_gate_pass(gate_pass_id)
        .await
        .map(Json)
}

// #[handler(session, permission = "quizzes::read")]
#[handler]
pub async fn find_gate_pass_handler(
    Path(gate_pass_id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .find_gate_pass(gate_pass_id)
        .await
        .map(Json)
}

#[handler]
pub async fn find_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(mut request): Payload<SearchGatePassRequest>,
) {
    state.repository.find_gate_passes(request).await.map(Json)
}

#[handler]
pub async fn find_sync_gate_passes_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(request): Payload<SyncGatePassRequest>,
) {
    state
        .repository
        .find_sync_gate_passes(request)
        .await
        .map(Json)
}

#[handler]
pub async fn generate_gate_pass_qr_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    state
        .repository
        .find_gate_pass(id)
        .await
        .map(|gate_pass| {
            qrcode_generator::to_svg_to_string(
                gate_pass.id.clone().as_bytes(),
                qrcode_generator::QrCodeEcc::Low,
                1024,
                None::<&str>,
            )
            .unwrap_or_default()
        })
        .map(|qr_svg| {
            Response::builder()
                .header(CONTENT_TYPE, "image/svg+xml")
                .body(qr_svg)
                .unwrap()
        })
}