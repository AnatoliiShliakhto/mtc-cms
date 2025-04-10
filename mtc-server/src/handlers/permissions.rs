use super::*;

#[handler(session, permission = "roles::read")]
pub async fn find_custom_permissions_handler(
    state: State<Arc<AppState>>,
) {
    state.repository.find_custom_permissions().await.map(Json)
}

#[handler(permission = "roles::write")]
pub async fn create_custom_permission_handler(
    Path(permission): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    let auth_user = session.get_auth_login().await?;

    state.repository
        .create_custom_permission(permission, auth_user)
        .await
}

#[handler(session, permission = "roles::delete")]
pub async fn delete_custom_permission_handler(
    Path(permission): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state.repository
        .delete_custom_permission(permission)
        .await
}