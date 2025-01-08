use super::*;

/// Get a list of all groups.
///
/// # Authorization
///
/// The user must have the [`PERMISSION_GROUPS_READ`] permission.
///
/// # Response
///
/// A JSON response with a list of groups as [`Vec`] of [`Entry`].
pub async fn find_group_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_READ).await?;

    state.repository.find_group_list().await?.to_response()
}

/// Get a group by ID.
///
/// # Authorization
///
/// The user must have the [`PERMISSION_GROUPS_READ`] permission.
///
/// # Arguments
///
/// * `id`: The ID of the group to retrieve. If [`ID_CREATE`], an empty group is returned that can be used to create a new group.
///
/// # Response
///
/// A JSON response with the requested [`Group`].
pub async fn find_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_READ).await?;

    let user = session.get_auth_login().await?;

    if id.eq(ID_CREATE) {
        Group {
            created_by: user.clone(),
            updated_by: user,
            .. Default::default()
        }
    } else {
        state.repository.find_group(id).await?
    }
        .to_response()
}

/// Update a group by ID.
///
/// # Authorization
///
/// The user must have the [`PERMISSION_GROUPS_WRITE`] permission.
///
/// # Arguments
///
/// * `id`: The ID of the group to update.
/// * `payload`: A JSON payload with the new values for the group.
///
/// # Response
///
/// A `200 OK` response if the group was updated successfully`.
pub async fn update_group_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_WRITE).await?;
    if payload.is_null() {  Err(GenericError::BadRequest)? }

    state.repository.update_group(payload, session.get_auth_login().await?).await?;

    Ok(StatusCode::OK)
}

/// Delete a group by ID.
///
/// # Authorization
///
/// The user must have the [`PERMISSION_GROUPS_DELETE`] permission.
///
/// # Arguments
///
/// * `id`: The ID of the group to delete.
///
/// # Response
///
/// A `200 OK` response if the group was deleted successfully.
pub async fn delete_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_GROUPS_DELETE).await?;

    state.repository.delete_group(id).await?;

    Ok(StatusCode::OK)
}