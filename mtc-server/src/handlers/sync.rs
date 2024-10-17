use super::*;

pub async fn sync_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let auth_id = session.get_auth_id().await?;

    session.set_expiry(Some(Expiry::OnInactivity(
        Duration::minutes(state.config.session_expiration)
    )));

    let req_id = payload.key_str("id").unwrap_or_default();

    let mut json_obj = Map::new();

    if auth_id.ne(&req_id) {
        let auth_state = session
            .get_auth_state()
            .await
            .unwrap_or_default();

        json_obj.insert("auth".into(), json!(auth_state));

        let user_custom_permissions = auth_state
            .permissions
            .iter()
            .map(|permission| permission
                    .split_once("::")
                    .unwrap_or((PERMISSION_PUBLIC, "")).0.to_owned().into())
            .collect::<BTreeSet<Cow<'static, str>>>();

        if auth_state.has_role(ROLE_WRITER) {
            let pages = state
                .repository.find_pages_entries(user_custom_permissions.clone()).await?;
            json_obj.insert("pages".into(), json!(pages));
        }

        let search_idx = state
            .repository
            .find_search_idx(user_custom_permissions).await?;

        json_obj.insert("search_idx".into(), json!(search_idx));
    }

    json_obj.to_response()
}
