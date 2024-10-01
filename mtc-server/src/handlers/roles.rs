use super::*;

pub async fn find_custom_role_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    let roles = state.repository.find_custom_role_list().await?;

    roles.to_response()
}

pub async fn find_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    let user = session.get_user().await?;

    let role = if id.eq(ID_CREATE) {
        Role {
            created_by: user.clone(),
            updated_by: user,
            .. Default::default()
        }
    } else {
        state.repository.find_role(id).await.unwrap_or_default()
    };

    let permissions = state.repository.find_permission_list().await?;
    let mut json_obj = json!(role);
    json_obj.insert_value("permissions_set", json!(permissions));

    json_obj.to_response()
}

pub async fn update_role_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_user().await?;

    state.repository.update_role(payload, by).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_DELETE).await?;

    state.repository.delete_role(id).await?;

    Ok(StatusCode::OK)
}