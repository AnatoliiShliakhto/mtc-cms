use super::*;

#[handler(session, permission = "groups::read")]
pub async fn find_group_list_handler(
    state: State<Arc<AppState>>,
) {
    state.repository.find_group_list().await.map(Json)
}

#[handler(permission = "groups::read")]
pub async fn find_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) {
    let user = session.get_auth_login().await?;

    if id.eq(ID_CREATE) {
        Ok(Json(Group {
            created_by: user.clone(),
            updated_by: user,
            ..Default::default()
        }))
    } else {
        state.repository.find_group(id).await.map(Json)
    }
}

#[handler(permission = "groups::write")]
pub async fn update_group_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) {
    if payload.is_null() { Err(GenericError::BadRequest)? }

    let login = session.get_auth_login().await?;

    state.repository.update_group(payload, login).await
}

#[handler(session, permission = "groups::delete")]
pub async fn delete_group_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state.repository.delete_group(id).await
}