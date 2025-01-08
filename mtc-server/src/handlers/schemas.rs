use super::*;

/// Handles requests to retrieve a list of schemas.
///
/// # Arguments
///
/// * `state` - Shared application state, encapsulating repository and other resources.
/// * `session` - Current user session, used to check permissions.
///
/// # Returns
///
/// * `JSON` - A result wrapping a response that contains the list of schemas as [`Vec`] of [`Entry`].
///
/// # Errors
///
/// Returns an error if the user does not have the required permissions or if there is a failure
/// retrieving the schema list.
pub async fn find_schema_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    state.repository.find_schema_list().await?.to_response()
}

/// Handles requests to retrieve a specific schema by its ID.
///
/// # Arguments
///
/// * `Path(id)` - The ID of the schema to retrieve, wrapped in a `Path`.
/// * `state` - Shared application state, encapsulating repository and other resources.
/// * `session` - Current user session, used to check permissions and retrieve authentication information.
///
/// # Returns
///
/// * `JSON` - A result wrapping a response that contains the [`Schema`] and its associated permissions.
///
/// # Errors
///
/// Returns an error if the user does not have the required permissions or if there is a failure retrieving the schema or permissions.
pub async fn find_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let user = session.get_auth_login().await?;

    let schema = if id.eq(ID_CREATE) {
        Schema {
            created_by: user.clone(),
            updated_by: user,
            .. Default::default()
        }
    } else {
        state.repository.find_schema(id).await?
    };

    let permissions = state.repository.find_custom_permissions().await?;
    let mut json_obj = json!(schema);
    json_obj.insert_value("permissions", json!(permissions));

    json_obj.to_response()
}

/// Handles requests to update a schema.
///
/// # Arguments
///
/// * `state` - Shared application state, encapsulating repository and other resources.
/// * `session` - Current user session, used to check permissions and retrieve authentication information.
/// * `Payload(payload)` - A payload containing the updated schema.
///
/// # Returns
///
/// * `HTTP Status Code 200` - The schema has been successfully updated.
///
/// # Errors
///
/// Returns an error if the user does not have the required permissions, if the payload is empty, or if there is a failure updating the schema.
pub async fn update_schema_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_schema(payload, by).await?;

    Ok(StatusCode::OK)
}

/// Handles requests to delete a schema.
///
/// # Arguments
///
/// * `Path(id)` - The ID of the schema to delete, wrapped in a `Path`.
/// * `state` - Shared application state, encapsulating repository and other resources.
/// * `session` - Current user session, used to check permissions and retrieve authentication information.
///
/// # Returns
///
/// * `HTTP Status Code 200` - The schema has been successfully deleted.
///
/// # Errors
///
/// Returns an error if the user does not have the required permissions or if there is a failure deleting the schema.
pub async fn delete_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_DELETE).await?;

    state.repository.delete_schema(id).await?;

    Ok(StatusCode::OK)
}