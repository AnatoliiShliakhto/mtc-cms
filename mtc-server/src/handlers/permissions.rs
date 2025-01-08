use super::*;

/// # Description
/// This function is used to find all custom permissions.
///
/// # Arguments
/// * `state` - The state of the application.
/// * `session` - The session of the user.
///
/// # Result
/// Returns a list of custom permissions as [`Vec`] of [`Cow<str>`].
///
/// # Errors
/// If the user does not have the necessary permissions, an error is returned.
pub async fn find_custom_permissions_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    let permissions = state.repository.find_custom_permissions().await?;
    
    permissions.to_response()
}

/// # Description
/// This function is used to create a new custom permission.
///
/// # Arguments
/// * `permission` - The name of the custom permission.
/// * `state` - The state of the application.
/// * `session` - The session of the user.
///
/// # Result
/// If the operation is successful, a `200 OK` response is returned.
///
/// # Errors
/// If the user does not have the necessary permissions, an error is returned.
pub async fn create_custom_permission_handler(
    Path(permission): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_WRITE).await?;

    let auth_user = session.get_auth_login().await?;

    state.repository
        .create_custom_permission(permission, auth_user)
        .await?;
    
    Ok(())
}

/// # Description
/// This function is used to delete a custom permission.
///
/// # Arguments
/// * `permission` - The name of the custom permission to delete.
/// * `state` - The state of the application.
/// * `session` - The session of the user.
///
/// # Result
/// If the operation is successful, a `200 OK` response is returned.
///
/// # Errors
/// If the user does not have the necessary permissions, an error is returned.
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