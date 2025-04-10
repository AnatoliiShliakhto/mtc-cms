use super::*;

#[handler]
pub async fn find_content_list_handler(
    Path(schema): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    let schema = state.repository.find_schema_by_slug(schema).await?;

    session.has_permission(&format!("{}::read", schema.permission)).await?;
    let is_writer = session.get_auth_state().await?.has_role(ROLE_WRITER);

    let content_list =
        state.repository.find_content_list(schema.slug, is_writer).await?;

    let mut response = json!({ "title": json!(schema.title) });
    response.as_object_mut().unwrap().insert("entries".to_string(), json!(content_list));

    Ok(Json(response))
}

#[handler]
pub async fn find_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) {
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

    let mut response = if session.get_auth_state().await?.has_role(ROLE_WRITER) {
        json!(content)
    } else {
        json!({ "data": content.data })
    };
    response.as_object_mut().unwrap().insert("title".to_string(), json!(content.title));
    response.as_object_mut().unwrap().insert("fields".to_string(), json!(content_schema.fields));

    Ok(Json(response))
}

#[handler]
pub async fn update_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) {
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

    state.repository.update_content(schema, slug, payload, by).await
}

#[handler]
pub async fn delete_content_handler(
    Path((schema, slug)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    let content_schema = match &*schema {
        "page"
        | "course" =>
            state.repository.find_schema_by_slug(slug.clone()).await?,
        _ =>
            state.repository.find_schema_by_slug(schema.clone()).await?,
    };

    session.has_permission(&format!("{}::delete", content_schema.permission)).await?;

    if content_schema.kind != SchemaKind::Pages { Err(GenericError::BadRequest)? }

    state.repository.delete_content(content_schema.slug, slug).await
}

#[handler]
pub async fn find_course_files_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) {
    let slug = payload.key_str("slug").unwrap_or_default();
    let schema = state.repository.find_schema_by_slug(slug).await?;

    session.has_permission(&format!("{}::read", schema.permission)).await?;

    state
        .repository
        .get_course_links(schema.slug)
        .await
        .map(Json)
}