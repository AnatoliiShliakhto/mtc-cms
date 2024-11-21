use super::*;

pub async fn find_group_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_READ).await?;

    let groups = state.repository.find_group_list().await?;

    groups.to_response()
}

pub async fn find_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_READ).await?;

    let user = session.get_auth_login().await?;

    let group = if id.eq(ID_CREATE) {
        Group {
            created_by: user.clone(),
            updated_by: user,
            .. Default::default()
        }
    } else {
        state.repository.find_group(id).await?
    };

    group.to_response()
}

pub async fn update_group_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_group(payload, by).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_DELETE).await?;

    state.repository.delete_group(id).await?;

    Ok(StatusCode::OK)
}