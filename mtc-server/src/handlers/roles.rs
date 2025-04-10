use super::*;

#[handler(session, permission = "roles::read")]
pub async fn find_custom_role_list_handler(
    state: State<Arc<AppState>>,
) {
    state.repository.find_custom_role_list().await.map(Json)
}

#[handler(permission = "roles::read")]
pub async fn find_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
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
    let mut response = json!(role);
    response.as_object_mut().unwrap().insert("permissions_set".to_string(), json!(permissions));

    Ok(Json(response))
}

#[handler(permission = "roles::write")]
pub async fn update_role_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) {
    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_role(payload, by).await
}

#[handler(session, permission = "roles::delete")]
pub async fn delete_role_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state.repository.delete_role(id).await
}