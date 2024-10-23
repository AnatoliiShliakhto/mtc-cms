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

        if auth_state.has_permission(PERMISSION_GROUPS_READ) {
            let groups = state.repository.find_group_list().await?;
            json_obj.insert("groups".into(), json!(groups));
        }

        if auth_state.has_permission(PERMISSION_ROLES_READ) {
            let roles = state.repository.find_role_list().await?;
            json_obj.insert("roles".into(), json!(roles));
        }

        let search_idx = state
            .repository
            .find_search_idx(user_custom_permissions).await?;

        json_obj.insert("search_idx".into(), json!(search_idx));

        if auth_id.ne(ROLE_ANONYMOUS) {
            let login = session.get_user().await?;
            state.repository.increment_user_access_count(login).await?;
        }
    }

    json_obj.to_response()
}