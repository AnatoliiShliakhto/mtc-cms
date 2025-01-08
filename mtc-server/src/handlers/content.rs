use super::*;

/// Handles the request to retrieve a list of content entries associated with a given schema.
///
/// Validates the user's read permissions for the specified schema and checks if the user
/// has the "writer" role to determine the visibility of unpublished content.
///
/// # Arguments
///
/// * `Path(schema)` - The schema slug to identify the content type.
/// * `state` - Shared application state, including the repository for database interactions.
/// * `session` - The current user session, used for permission and role checks.
///
/// # Returns
///
/// Returns a JSON response containing the schema **title** and a list of content entries [`Vec`] of [`Content`].
pub async fn find_content_list_handler(
    Path(schema): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let schema = state.repository.find_schema_by_slug(schema).await?;

    session.has_permission(&format!("{}::read", schema.permission)).await?;
    let is_writer = session.get_auth_state().await?.has_role(ROLE_WRITER);

    let content_list =
        state.repository.find_content_list(schema.slug, is_writer).await?;

    let mut json_obj = json!({ "title": schema.title });
    json_obj.insert_value("entries", json!(content_list));
    json_obj.to_response()
}

/// Handles the request to retrieve a content entry associated with a given schema and slug.
///
/// Validates the user's read permissions for the specified schema and checks if the user
/// has the "writer" role to determine the visibility of unpublished content.
///
/// # Arguments
///
/// * `Path((schema, slug))` - The schema slug and content entry slug to identify the content type.
/// * `state` - Shared application state, including the repository for database interactions.
/// * `session` - The current user session, used for permission and role checks.
///
/// # Returns
///
/// Returns a JSON response containing the [`Content`] **entry**, including its **title** and **fields** as Vec<[`Field`]>.
pub async fn find_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let content_schema = match &*schema {
        "page"
        | "course" =>
            state.repository.find_schema_by_slug(slug.clone()).await?,
        _ =>
            state.repository.find_schema_by_slug(schema.clone()).await?,
    };

    session.has_permission(&format!("{}::read", content_schema.permission)).await?;

    let content = if slug.eq(ID_CREATE) {
        Content::default()
    } else {
        state.repository.find_content(schema, slug).await?
    };

    let mut json_obj = if session.get_auth_state().await?.has_role(ROLE_WRITER) {
        json!(content)
    } else {
        json!({ "data": content.data })
    };
    json_obj.insert_value("title", json!(content.title));
    json_obj.insert_value("fields", json!(content_schema.fields));

    json_obj.to_response()
}

/// Handles the request to update a content entry associated with a given schema.
///
/// Validates the user's write permissions for the specified schema and checks if the user
/// has the "writer" role to determine the visibility of unpublished content.
///
/// # Arguments
///
/// * `Path((schema, slug))` - The schema slug and content entry slug to identify the content type.
/// * `state` - Shared application state, including the repository for database interactions.
/// * `session` - The current user session, used for permission and role checks.
/// * `Payload(payload)` - The JSON payload containing the updated content entry.
///
/// # Returns
///
/// Returns a `200 OK status code` if the content entry is successfully updated.
/// An error is returned if the content entry is not found or if the user does not have permission
/// to update the content entry.
pub async fn update_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let content_schema = match &*schema {
        "page"
        | "course" =>
            state.repository.find_schema_by_slug(slug.clone()).await?,
        _ =>
            state.repository.find_schema_by_slug(schema.clone()).await?,
    };

    session.has_permission(&format!("{}::write", content_schema.permission)).await?;

    let id = payload.key_str("id").unwrap_or_default();
    if (id.is_empty() | id.eq(ID_CREATE)) &&
        (content_schema.kind == SchemaKind::Page || content_schema.kind == SchemaKind::Course) {
        Err(GenericError::BadRequest)?
    }

    if !id.is_empty() & payload.is_null() { Err(GenericError::BadRequest)? }

    let by = session.get_auth_login().await?;

    state.repository.update_content(schema, slug, payload, by).await?;

    Ok(StatusCode::OK)
}

/// Handles the request to delete a content entry associated with a given schema and slug.
///
/// Validates the user's delete permissions for the specified schema and checks if the user
/// has the "writer" role to determine the visibility of unpublished content.
///
/// # Arguments
///
/// * `Path((schema, slug))` - The schema slug and content entry slug to identify the content type.
/// * `state` - Shared application state, including the repository for database interactions.
/// * `session` - The current user session, used for permission and role checks.
///
/// # Returns
///
/// Returns `200 OK status code` if the content entry is successfully deleted.
pub async fn delete_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let content_schema = match &*schema {
        "page"
        | "course" =>
            state.repository.find_schema_by_slug(slug.clone()).await?,
        _ =>
            state.repository.find_schema_by_slug(schema.clone()).await?,
    };

    session.has_permission(&format!("{}::delete", content_schema.permission)).await?;

    if content_schema.kind != SchemaKind::Pages { Err(GenericError::BadRequest)? }

    state.repository.delete_content(content_schema.slug, slug).await?;

    Ok(StatusCode::OK)
}

/// Handles the request to retrieve course files linked to a specific course schema.
///
/// Validates the user's read permissions for the specified course schema
/// before fetching the associated course files.
///
/// # Arguments
///
/// * `state` - Shared application state, including the repository for database interactions.
/// * `session` - The current user session, used for permission checks.
/// * `Payload(payload)` - The request payload containing the course slug.
///
/// # Returns
///
/// Returns a JSON response containing the list of course files: [`Vec`] of [`FileEntry`].
pub async fn find_course_files_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let slug = payload.key_str("slug").unwrap_or_default();
    let schema = state.repository.find_schema_by_slug(slug).await?;

    session.has_permission(&format!("{}::read", schema.permission)).await?;

    state
        .repository
        .get_course_links(schema.slug)
        .await?
        .to_response()
}