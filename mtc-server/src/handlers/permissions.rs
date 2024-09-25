use super::*;

pub async fn find_custom_permissions_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    let permissions = state.repository.find_custom_permissions().await?;
    
    permissions.to_response()
}

pub async fn create_custom_permission_handler(
    Path(permission): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_WRITE).await?;

    let auth_user = session.get_user().await?;

    state.repository
        .create_custom_permission(permission, auth_user)
        .await?;
    
    Ok(())
}

pub async fn delete_custom_permission_handler(
    Path(permission): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_DELETE).await?;

    state.repository
        .delete_custom_permission(permission)
        .await?;

    Ok(())
}