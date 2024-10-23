use super::*;

pub async fn find_schema_list_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let schemas = state.repository.find_schema_list().await?;

    schemas.to_response()
}

pub async fn find_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let user = session.get_user().await?;

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

pub async fn update_schema_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_user().await?;

    state.repository.update_schema(payload, by).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_DELETE).await?;

    state.repository.delete_schema(id).await?;

    Ok(StatusCode::OK)
}