use super::*;

pub async fn find_user_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let access = session.get_access_state().await?;

    let users = state.repository.find_user_list(access).await?;

    users.to_response()
}

pub async fn find_user_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let user = session.get_user().await?;
    let access = session.get_access_state().await?;

    let user = if id.eq(ID_CREATE) {
        User {
            created_by: user.clone(),
            updated_by: user,
            ..Default::default()
        }
    } else {
        state.repository.find_user(id.clone(), access).await.unwrap_or_default()
    };

    let user_roles = state.repository.find_roles_ids_by_user_id(id).await?;
    let roles = state.repository.find_role_list().await?;
    let groups = state.repository.find_group_list().await?;
    let mut json_obj = json!(user);
    json_obj.insert_value("roles", json!(user_roles));
    json_obj.insert_value("roles_set", json!(roles));
    json_obj.insert_value("groups_set", json!(groups));

    json_obj.to_response()
}

pub async fn update_user_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_user().await?;

    state.repository.update_user(payload, by, state.config.password_salt.clone()).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_user_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_DELETE).await?;

    state.repository.delete_user(id).await?;

    Ok(StatusCode::OK)
}