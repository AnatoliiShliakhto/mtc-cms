use super::*;

#[handler(session, permission = "schemas::read")]
pub async fn find_schema_list_handler(
    state: State<Arc<AppState>>,
) {
    state.repository.find_schema_list().await.map(Json)
}

#[handler(permission = "schemas::read")]
pub async fn find_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
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
    let mut response = json!(schema);
    response.as_object_mut().unwrap().insert("permissions".to_string(), json!(permissions));
    
    Ok(Json(response))
}

#[handler(permission = "schemas::write")]
pub async fn update_schema_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) {
    session.has_permission(PERMISSION_SCHEMAS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_schema(payload, by).await
}

#[handler(session, permission = "schemas::delete")]
pub async fn delete_schema_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    state.repository.delete_schema(id).await
}