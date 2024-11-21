use super::*;

pub async fn sync_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let auth_id = session.get_auth_id().await?;

    let mut json_obj = Map::new();

    let auth_state = if auth_id.eq(ROLE_ANONYMOUS) | auth_id.is_empty() {
        AuthState::default()
    } else if let Ok(user) = state
        .repository
        .find_user(auth_id.clone(), Access::administrator()
        ).await {
        let user_state = state.repository.find_user_state(user.id).await?;

        let auth_state = AuthState::from(user_state.clone());
        let access = if auth_state.is_admin() {
            Access::administrator()
        } else {
            Access::from(user_state)
        };
        state.repository.increment_user_access_count(auth_state.login.clone()).await?;
        session.set_state(&auth_state, &access).await?;

        auth_state
    } else {
        AuthState::default()
    };

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

    json_obj.to_response()
}