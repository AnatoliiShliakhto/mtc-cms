use super::*;

pub async fn find_content_list_handler(
    Path(schema): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let schema = state.repository.find_schema_by_slug(schema).await?;

    session.has_permission(&format!("{}::read", schema.permission)).await?;
    let full = session.get_auth_state().await?.has_role(ROLE_WRITER);

    let content_list =
        state.repository.find_content_list(schema.slug, full).await?;
    let mut json_obj = json!({ "title": schema.title });
    json_obj.insert_value("entries", json!(content_list));

    json_obj.to_response()
}

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

    if !id.is_empty() & payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_user().await?;

    state.repository.update_content(schema, slug, payload, by).await?;

    Ok(StatusCode::OK)
}

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

    if content_schema.kind != SchemaKind::Pages {
        Err(GenericError::BadRequest)?
    }

    state.repository.delete_content(content_schema.slug, slug).await?;

    Ok(StatusCode::OK)
}