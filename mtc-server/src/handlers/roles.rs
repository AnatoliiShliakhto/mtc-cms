use super::*;

/// # Description
///
/// Finds all custom roles and returns them as a JSON list.
///
/// # Permissions
///
/// - [`PERMISSION_ROLES_READ`]
///
/// # Errors
///
/// - `GenericError::Unauthorized` if the user lacks the `roles::read` permission
///
/// # Returns
///
/// - A JSON response containing a list of custom roles as [`Vec`] of [`Entry`]
pub async fn find_custom_role_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    state.repository.find_custom_role_list().await?.to_response()
}

/// # Description
///
/// Retrieves a role by its ID and returns it as a JSON object along with the list of associated permissions.
///
/// # Parameters
///
/// - `id`: The ID of the role to retrieve. If the ID is `ID_CREATE`, a default role is created.
/// - `state`: The application state containing shared resources like the repository.
/// - `session`: The session information for the current user, used to check permissions and identify the user.
///
/// # Permissions
///
/// - [`PERMISSION_ROLES_READ`] permission is required to execute this function.
///
/// # Errors
///
/// - `GenericError::Unauthorized` if the user lacks the `roles::read` permission.
/// - `DatabaseError::EntryNotFound` if no role is found with the specified ID.
///
/// # Returns
///
/// - A JSON response containing the [`Role`] details and associated permissions.
pub async fn find_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_READ).await?;

    let user = session.get_auth_login().await?;

    let role = if id.eq(ID_CREATE) {
        Role {
            created_by: user.clone(),
            updated_by: user,
            .. Default::default()
        }
    } else {
        state.repository.find_role(id).await?
    };

    let permissions = state.repository.find_permission_list().await?;
    let mut json_obj = json!(role);
    json_obj.insert_value("permissions_set", json!(permissions));

    json_obj.to_response()
}

/// # Description
///
/// Updates a role with the specified ID.
///
/// # Parameters
///
/// - `state`: The application state containing shared resources like the repository.
/// - `session`: The session information for the current user, used to check permissions and identify the user.
/// - `payload`: The JSON payload containing the role details to update. If the ID is `ID_CREATE`, a new role is created.
///
/// # Permissions
///
/// - [`PERMISSION_ROLES_WRITE`] permission is required.
///
/// # Errors
///
/// - `GenericError::Unauthorized` if the user lacks the `roles::write` permission.
/// - `DatabaseError::EntryNotFound` if no role is found with the specified ID.
/// - `GenericError::BadRequest` if the payload is null.
pub async fn update_role_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_role(payload, by).await?;

    Ok(StatusCode::OK)
}

/// # Description
///
/// Deletes a role with the specified ID and returns the success status.
///
/// # Parameters
///
/// - `id`: The ID of the role to delete.
/// - `state`: The application state containing shared resources like the repository.
/// - `session`: The session information for the current user, used to check permissions and identify the user.
///
/// # Permissions
///
/// - [`PERMISSION_ROLES_DELETE`] permission is required.
///
/// # Errors
///
/// - `GenericError::Unauthorized` if the user lacks the `roles::delete` permission.
/// - `DatabaseError::EntryNotFound` if no role is found with the specified ID.
pub async fn delete_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_ROLES_DELETE).await?;

    state.repository.delete_role(id).await?;

    Ok(StatusCode::OK)
}