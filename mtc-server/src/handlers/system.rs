use super::*;

pub async fn find_system_info_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let system_info = state.repository.find_system_info().await?;
    let migrations = state.repository.find_migrations().await?;
    let sitemap = state.repository.get_system_value("sitemap".into()).await?;
    let mut json_obj = json!({});
    json_obj.insert_value("info", json!(system_info));
    json_obj.insert_value("migrations", json!(migrations));
    json_obj.insert_value("sitemap", json!(sitemap));

    json_obj.to_response()
}

pub async fn migration_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let mut migrations = state.repository.find_migrations().await?;
    let mut login = session.get_auth_login().await.unwrap_or_default();
    let mut password = Cow::Borrowed("");

    if migrations.is_empty() {
        let pwd = payload.key_str("password").unwrap_or(ROLE_ADMINISTRATOR.into());
        let salt = SaltString::from_b64(&state.config.password_salt).unwrap();

        let argon2 = Argon2::default();
        let Ok(password_hash) = argon2.hash_password(pwd.as_bytes(), &salt) else {
            Err(SessionError::PasswordHash)?
        };
        password = Cow::Owned(password_hash.to_string());
        login = payload.key_str("login").unwrap_or(ROLE_ADMINISTRATOR.into());
    } else {
        if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
            Err(SessionError::AccessForbidden)?
        }
    }

    let migration_files = state.repository.get_migration_files().await?
        .iter()
        .filter(|value| !migrations.contains(value.as_ref()))
        .cloned()
        .collect::<BTreeSet<Cow<'static, str>>>();

    for file in migration_files.iter() {
        let sql = state.repository.get_migration_file(file).await?;

        state.repository.migrate(sql, login.clone(), password.clone()).await?;
        info!("Migration {} is done!", file);

        migrations.insert(file.clone());

        state.repository.update_migrations(migrations.clone()).await?;
    }

    Ok(StatusCode::OK)
}

pub async fn search_idx_rebuild_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    state.repository.rebuild_search_idx().await?;

    Ok(())
}

///todo RegEx sanitizer
pub async fn search_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let Some(payload) = payload.as_str() else {
        Err(GenericError::BadRequest)?
    };
    let payload = payload.to_string();

    let auth_state = session
        .get_auth_state()
        .await
        .unwrap_or_default();

    let user_custom_permissions = auth_state
        .permissions
        .iter()
        .map(|permission| permission
            .split_once("::")
            .unwrap_or((PERMISSION_PUBLIC, "")).0.to_owned().into())
        .collect::<BTreeSet<Cow<'static, str>>>();

    let search_idx = state
        .repository
        .search_content(payload.into(), user_custom_permissions)
        .await?;

    search_idx.to_response()
}

pub async fn sitemap_build_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    state.repository.sitemap_build().await?;

    Ok(())
}

pub async fn health_handler() -> Result<impl IntoResponse> {
    Ok(StatusCode::OK)
}